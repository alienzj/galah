extern crate galah;

extern crate clap;
use clap::*;
use std::env;

extern crate log;

extern crate bird_tool_utils;
use bird_tool_utils::clap_utils::*;

static PROGRAM_NAME: &str = "Galah";

fn main() {
    let app = build_cli();
    let matches = app.clone().get_matches();
    set_log_level(&matches, false, PROGRAM_NAME, crate_version!());

    match matches.subcommand_name() {
        Some("cluster") => {
            galah::cluster_argument_parsing::run_cluster_subcommand(
                &matches,
                "galah",
                crate_version!(),
            );
        }
        Some("cluster-validate") => {
            let m = matches.subcommand_matches("cluster-validate").unwrap();
            set_log_level(m, true, PROGRAM_NAME, crate_version!());

            let num_threads = value_t!(m.value_of("threads"), usize).unwrap();
            rayon::ThreadPoolBuilder::new()
                .num_threads(num_threads)
                .build_global()
                .expect("Programming error: rayon initialised multiple times");

            let ani = galah::cluster_argument_parsing::parse_percentage(&m, "ani");
            let min_aligned_fraction =
                galah::cluster_argument_parsing::parse_percentage(&m, "min-aligned-fraction");
            let fraglen = value_t!(m, "fraglen", u32);

            galah::cluster_validation::validate_clusters(
                m.value_of("cluster-file").unwrap(),
                ani.unwrap().unwrap(),
                min_aligned_fraction.unwrap().unwrap(),
                fraglen.unwrap(),
            );
        }
        _ => panic!("Programming error"),
    }
}

fn build_cli() -> App<'static, 'static> {
    let mut app = App::new("galah")
        .version(crate_version!())
        .author("Ben J. Woodcroft <benjwoodcroft near gmail.com>")
        .about("Metagenome assembled genome (MAG) dereplicator / clusterer")
        .args_from_usage(
            "-v, --verbose       'Print extra debug logging information'
             -q, --quiet         'Unless there is an error, do not print logging information'",
        )
        .global_setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("cluster-validate")
                .about("Verify clustering results")
                .arg(
                    Arg::with_name("cluster-file")
                        .long("cluster-file")
                        .required(true)
                        .help("Output of 'cluster' subcommand")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("ani")
                        .long("ani")
                        .default_value("99")
                        .help("ANI to validate against")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("min-aligned-fraction")
                        .long("min-aligned-fraction")
                        .help("Min aligned fraction of two genomes for clustering")
                        .takes_value(true)
                        .default_value("50"),
                )
                .arg(
                    Arg::with_name("threads")
                        .short("t")
                        .long("threads")
                        .default_value("1")
                        .takes_value(true),
                ),
        );

    // .subcommand(
    //     SubCommand::with_name("dist")
    //         .about("Calculate pairwise distances between a set of genomes")

    //         .arg(Arg::with_name("checkm-tab-table")
    //             .long("checkm-tab-table")
    //             .required(true)
    //             .help("Output of CheckM lineage_wf/taxonomy_wf/qa with --tab_table specified")
    //             .takes_value(true))
    //         .arg(Arg::with_name("genome-fasta-files")
    //             .long("genome-fasta-files")
    //             .multiple(true)
    //             .required(true)
    //             .takes_value(true))
    //         .arg(Arg::with_name("num-hashes")
    //             .long("num-hashes")
    //             .takes_value(true)
    //             .default_value("1000"))
    //         .arg(Arg::with_name("kmer-length")
    //             .long("kmer-length")
    //             .takes_value(true)
    //             .default_value("21"))
    //         .arg(Arg::with_name("threads")
    //             .short("-t")
    //             .long("threads")
    //             .default_value("1")
    //             .takes_value(true)))

    app = galah::cluster_argument_parsing::add_cluster_subcommand(app);
    return app;
}
