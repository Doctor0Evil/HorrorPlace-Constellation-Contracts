// Target repo: Doctor0Evil/HorrorPlace-Codebase-of-Death
// Binary: floor_bias_report
//
// cargo run --bin floor_bias_report -- --db-path constellationindex.db --floor-id B1

use rusqlite::{Connection, named_params};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
    /// Path to constellationindex.db
    #[structopt(long)]
    db_path: String,

    /// Floor ID to report (e.g., B1)
    #[structopt(long)]
    floor_id: String,

    /// Window around each anchor t to aggregate usage (0.0 - 0.5 typical)
    #[structopt(long, default_value = "0.10")]
    t_window: f32,

    /// Minimum total uses for a pattern to be shown (noise filter)
    #[structopt(long, default_value = "5")]
    min_uses: u32,
}

#[derive(Debug)]
struct AnchorRow {
    region_id: String,
    region_tag: String,
    t: f32,
    w_energy: f32,
    w_quality: f32,
    w_safety: f32,
    horror_bias: Option<String>,
}

#[derive(Debug)]
struct PatternUsage {
    patternname: String,
    total_uses: u32,
}

fn main() -> anyhow::Result<()> {
    let args = Args::from_args();
    let conn = Connection::open(&args.db_path)?;

    let anchors = load_anchors(&conn, &args.floor_id)?;
    if anchors.is_empty() {
        println!(
            "No floor_region_anchors found for floor_id = {}",
            args.floor_id
        );
        return Ok(());
    }

    println!("Horror bias profile for floor {}\n", args.floor_id);

    for anchor in anchors {
        let usage = load_pattern_usage_near_anchor(
            &conn,
            &args.floor_id,
            anchor.t,
            args.t_window,
            args.min_uses,
        )?;

        print_anchor_report(&anchor, &usage);
    }

    Ok(())
}

fn load_anchors(conn: &Connection, floor_id: &str) -> anyhow::Result<Vec<AnchorRow>> {
    let mut stmt = conn.prepare(
        r#"
        SELECT region_id, region_tag, t,
               w_energy, w_quality, w_safety, horror_bias
        FROM floor_region_anchors
        WHERE floor_id = :floor_id
        ORDER BY t ASC
        "#,
    )?;

    let rows = stmt
        .query_map(named_params! { ":floor_id": floor_id }, |row| {
            Ok(AnchorRow {
                region_id: row.get("region_id")?,
                region_tag: row.get("region_tag")?,
                t: row.get::<_, f32>("t")?,
                w_energy: row.get("w_energy")?,
                w_quality: row.get("w_quality")?,
                w_safety: row.get("w_safety")?,
                horror_bias: row.get("horror_bias")?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(rows)
}

/// Loads pattern usage near a given anchor t, within ±t_window.
/// Assumes bcirequestframe has floor_id and t_norm, and bcibindinggeometry has patternname.
fn load_pattern_usage_near_anchor(
    conn: &Connection,
    floor_id: &str,
    anchor_t: f32,
    t_window: f32,
    min_uses: u32,
) -> anyhow::Result<Vec<PatternUsage>> {
    let mut stmt = conn.prepare(
        r#"
        WITH usage AS (
          SELECT
              bg.patternname AS patternname,
              COUNT(*) AS uses
          FROM bcirequestframe rf
          JOIN bcibindinggeometry bg ON bg.frameid = rf.frameid
          WHERE rf.floor_id = :floor_id
            AND rf.t_norm BETWEEN :t_min AND :t_max
          GROUP BY bg.patternname
        )
        SELECT patternname, uses
        FROM usage
        WHERE uses >= :min_uses
        ORDER BY uses DESC;
        "#,
    )?;

    let t_min = (anchor_t - t_window).max(0.0);
    let t_max = (anchor_t + t_window).min(1.0);

    let rows = stmt
        .query_map(
            named_params! {
                ":floor_id": floor_id,
                ":t_min": t_min,
                ":t_max": t_max,
                ":min_uses": min_uses as i64,
            },
            |row| {
                Ok(PatternUsage {
                    patternname: row.get("patternname")?,
                    total_uses: row.get::<_, i64>("uses")? as u32,
                })
            },
        )?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(rows)
}

fn print_anchor_report(anchor: &AnchorRow, usage: &[PatternUsage]) {
    println!(
        "Region {} ({}) at t = {:.2}",
        anchor.region_id, anchor.region_tag, anchor.t
    );
    println!(
        "  Intended weights: energy = {:.2}, quality = {:.2}, safety = {:.2}",
        anchor.w_energy, anchor.w_quality, anchor.w_safety
    );
    if let Some(bias) = &anchor.horror_bias {
        println!("  Horror bias tag: {}", bias);
    }

    if usage.is_empty() {
        println!("  Observed patterns: (no data above threshold)\n");
        return;
    }

    println!("  Observed patterns near this anchor:");
    for p in usage {
        println!("    - {:<24} uses: {}", p.patternname, p.total_uses);
    }
    println!();
}
