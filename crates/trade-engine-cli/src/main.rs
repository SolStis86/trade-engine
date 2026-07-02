use alpaca_ingest::{AlpacaClient, AlpacaConfig, FetchStockBarsRequest};
use anyhow::Context;
use chrono::{DateTime, Utc};
use clap::{Parser, Subcommand};
use tokio::io::AsyncWriteExt;

#[derive(Debug, Parser)]
#[command(name = "trade-engine")]
#[command(about = "Trade Engine research and ingestion CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Ingest(IngestCommand),
}

#[derive(Debug, Parser)]
struct IngestCommand {
    #[command(subcommand)]
    command: IngestSubcommand,
}

#[derive(Debug, Subcommand)]
enum IngestSubcommand {
    HistoricalAlpacaStockBars(HistoricalAlpacaStockBarsArgs),
}

#[derive(Debug, Parser)]
struct HistoricalAlpacaStockBarsArgs {
    #[arg(long)]
    symbol: String,

    #[arg(long)]
    timeframe: String,

    #[arg(long)]
    start: DateTime<Utc>,

    #[arg(long)]
    end: DateTime<Utc>,

    #[arg(long)]
    feed: Option<String>,

    #[arg(long)]
    adjustment: Option<String>,

    #[arg(long, default_value_t = 10_000)]
    limit: u32,

    #[arg(long)]
    output: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "trade_engine=info,alpaca_ingest=info".into()),
        )
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Ingest(command) => match command.command {
            IngestSubcommand::HistoricalAlpacaStockBars(args) => ingest_historical_alpaca_stock_bars(args).await?,
        },
    }

    Ok(())
}

async fn ingest_historical_alpaca_stock_bars(args: HistoricalAlpacaStockBarsArgs) -> anyhow::Result<()> {
    let config = AlpacaConfig::from_env().context("failed to load Alpaca config from environment")?;
    let client = AlpacaClient::new(config).context("failed to create Alpaca client")?;

    let request = FetchStockBarsRequest {
        symbol: args.symbol,
        timeframe: args.timeframe,
        start: args.start,
        end: args.end,
        feed: args.feed,
        adjustment: args.adjustment,
        limit: args.limit,
    };

    request.validate()?;

    let candles = client
        .fetch_stock_bars(request)
        .await
        .context("failed to fetch stock bars from Alpaca")?;

    let output = args.output.unwrap_or_else(|| "-".to_string());

    if output == "-" {
        for candle in candles {
            println!("{}", serde_json::to_string(&candle)?);
        }
    } else {
        let mut file = tokio::fs::File::create(&output)
            .await
            .with_context(|| format!("failed to create output file: {output}"))?;

        for candle in candles {
            let line = serde_json::to_string(&candle)?;
            file.write_all(line.as_bytes()).await?;
            file.write_all(b"\n").await?;
        }
    }

    Ok(())
}
