use clap::{Arg, ArgGroup, Command};
use clockwork_client::http::state::HttpMethod;
use solana_sdk::{pubkey::Pubkey, signature::Keypair};

#[derive(Debug, PartialEq)]
pub enum CliCommand {
    // API commands
    ApiNew {
        ack_authority: Pubkey,
        base_url: String,
    },

    // Config commands
    ConfigGet,
    ConfigSet {
        admin: Option<Pubkey>,
        worker_fee: Option<u64>,
        grace_period: Option<i64>,
        spam_penalty: Option<u64>,
    },

    // Task commands
    TaskGet {
        address: Pubkey,
    },

    // Http
    HttpRequestNew {
        api: Pubkey,
        id: String,
        method: HttpMethod,
        route: String,
    },

    // Admin commands
    Initialize {
        mint: Pubkey,
    },

    // Node commands
    NodeGet {
        worker: Pubkey,
    },
    NodeRegister {
        worker: Keypair,
    },
    NodeStake {
        amount: u64,
        worker: Pubkey,
    },

    // Pool commands
    PoolGet,

    // Queue commands
    QueueCreate {
        name: String,
        schedule: String,
    },
    QueueGet {
        address: Pubkey,
        task_id: Option<u64>,
    },

    // Registry
    RegistryGet,

    // Snapshot
    SnapshotGet {
        entry_id: Option<u64>,
    },

    // Utility commands
    Clock,
    Health,
}

pub fn app() -> Command<'static> {
    Command::new("Clockwork")
        .bin_name("clockwork")
        .about("Automation infrastructure for Solana")
        .version(version!())
        .arg_required_else_help(true)
        .subcommand(
            Command::new("api")
                .about("Manage APIs registered with the HTTP program")
                .arg_required_else_help(true)
                .subcommand(Command::new("new")
                    .about("Register a new api")
                    .arg(
                        Arg::new("ack_authority")
                            .long("ack_authority")
                            .short('a')
                            .takes_value(true)
                            .required(true)
                            .help("The authority which will acknowledge requests sent to this API"),
                    )
                    .arg(
                        Arg::new("base_url")
                            .long("base_url")
                            .short('b')
                            .takes_value(true)
                            .required(true)
                            .help("The base url of the API"),
                    )
                )
        )
        .subcommand(Command::new("clock").about("Display the current Solana clock time"))
        .subcommand(
            Command::new("config")
                .about("Manage the Clockwork configs")
                .arg_required_else_help(true)
                .subcommand(Command::new("get").about("Get a config value"))
                .subcommand(
                    Command::new("set")
                        .about("Set a config value")
                        .arg(
                            Arg::new("admin")
                                .long("admin")
                                .value_name("PUBKEY")
                                .takes_value(true),
                        )
                        .arg(
                            Arg::new("worker_fee")
                                .long("worker_fee")
                                .value_name("NUM_LAMPORTS")
                                .takes_value(true),
                        )
                        .arg(
                            Arg::new("grace_period")
                                .long("grace_period")
                                .value_name("NUM_SECONDS")
                                .takes_value(true),
                        )
                        .arg(
                            Arg::new("spam_penalty")
                                .long("spam_penalty")
                                .value_name("NUM_LAMPORTS")
                                .takes_value(true),
                        )
                        .group(
                            ArgGroup::new("config_settings")
                                .args(&[
                                    "admin",
                                    "worker_fee",
                                    "grace_period",
                                    "spam_penalty",
                                ])
                                .multiple(true),
                        ),
                ),
        )
        .subcommand(Command::new("health").about("Get the current system health"))
        .subcommand(
            Command::new("http")
                .about("Trigger HTTP requests from Solana")
                .arg(
                    Arg::new("api")
                        .long("api")
                        .short('a')
                        .takes_value(true)
                        .required(true)
                        .help("The address of the API to send this request to"),
                )
                .arg(
                    Arg::new("id")
                        .long("id")
                        .short('i')
                        .takes_value(true)
                        .required(true)
                        .help("A deduplication id for the request"),
                )
                .arg(
                    Arg::new("method")
                        .long("method")
                        .short('m')
                        .takes_value(true)
                        .required(true)
                        .help("The method to invoke the HTTP request with (GET, POST, PATCH)"),
                )
                .arg(
                    Arg::new("route")
                        .long("route")
                        .short('r')
                        .takes_value(true)
                        .required(true)
                        .help("The relative route to send the HTTP request to"),
                ),
        )
        .subcommand(
            Command::new("initialize")
                .about("Initialize the Clockwork programs")
                .arg(
                    Arg::new("mint")
                        .long("mint")
                        .short('m')
                        .takes_value(true)
                        .required(true)
                        .help("Mint address of network token"),
                ),
        )
        .subcommand(
            Command::new("task")
                .about("Manage an task")
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("get").about("Get an task").arg(
                        Arg::new("address")
                            .index(1)
                            .takes_value(true)
                            .required(true)
                            .help("Public address of a task"),
                    ),
                ),
        )
        .subcommand(
            Command::new("node")
                .about("Manage your nodes")
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("get")
                        .about("Get a node by worker address")
                        .arg(
                            Arg::new("worker")
                                .index(1)
                                .takes_value(true)
                                .required(true)
                                .help("The worker address to stake tokens with"),
                        ),
                )
                .subcommand(
                    Command::new("register")
                        .about("Register a new node with the Clockwork network")
                        .arg(
                            Arg::new("worker")
                                .index(1)
                                .takes_value(true)
                                .required(true)
                                .help("Filepath to the worker keypair"),
                        ),
                )
                .subcommand(
                    Command::new("stake")
                        .about("Stake CRON with your Solana node")
                        .arg(
                            Arg::new("amount")
                                .index(1)
                                .takes_value(true)
                                .required(true)
                                .help("The number of tokens to stake"),
                        )
                        .arg(
                            Arg::new("worker")
                                .index(2)
                                .takes_value(true)
                                .required(true)
                                .help("The worker address to stake tokens with"),
                        ),
                ),
        )
        .subcommand(Command::new("pool").about("Get the worker pool info"))
        .subcommand(
            Command::new("queue")
                .about("Manage your queues")
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("create")
                        .about("Create a new queue")
                        .arg(
                            Arg::new("filepath")
                                .long("filepath")
                                .short('f')
                                .takes_value(true)
                                .required(true)
                                .help("Filepath to the instruction to invoke"),
                        )
                        .arg(
                            Arg::new("name")
                                .long("name")
                                .short('n')
                                .takes_value(true)
                                .required(true)
                                .help("The name of the queue"),
                        )
                        .arg(
                            Arg::new("schedule")
                                .long("schedule")
                                .short('s')
                                .takes_value(true)
                                .required(false)
                                .help("Schedule to invoke the instruction"),
                        ),
                )
                .subcommand(
                    Command::new("get")
                        .about("Get a queue")
                        .arg(
                            Arg::new("address")
                                .index(1)
                                .takes_value(true)
                                .required(false)
                                .help("Public address of a queue"),
                        )
                        .subcommand(
                            Command::new("task")
                                .about("Lookup a task in the queue")
                                .arg(
                                    Arg::new("id")
                                        .index(1)
                                        .takes_value(true)
                                        .required(true)
                                        .help("The id of a task in the queue"),
                                ),
                        ),
                ),
        )
        .subcommand(Command::new("registry").about("Get the registry account"))
        .subcommand(
            Command::new("snapshot")
                .about("Lookup the current snapshot")
                .subcommand(
                    Command::new("get")
                        .about("Get the snapshot account data")
                        .subcommand(
                            Command::new("entry")
                                .about("Lookup an entry in the snapshot")
                                .arg(
                                    Arg::new("id")
                                        .index(1)
                                        .takes_value(true)
                                        .required(true)
                                        .help("The id of an entry in the snapshot"),
                                ),
                        ),
                ),
        )
}
