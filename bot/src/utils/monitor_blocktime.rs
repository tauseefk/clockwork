use {
    crate::env,
    solana_account_decoder::UiAccountEncoding,
    solana_client::{pubsub_client::PubsubClient, rpc_config::RpcAccountInfoConfig},
    solana_sdk::{
        account::Account,
        clock::{Clock, Epoch, Slot, UnixTimestamp},
        commitment_config::CommitmentConfig,
        pubkey::Pubkey,
    },
    std::{
        str::FromStr,
        sync::mpsc::{self, Receiver},
        thread,
    },
};

pub fn monitor_blocktime() -> Receiver<i64> {
    let (blocktime_sender, blocktime_receiver) = mpsc::channel::<i64>();
    thread::spawn(move || {
        let mut latest_blocktime: i64 = 0;
        let clock_addr = Pubkey::from_str("SysvarC1ock11111111111111111111111111111111").unwrap();
        let (_ws_client, clock_receiver) = PubsubClient::account_subscribe(
            env::wss_endpoint().as_str().into(),
            &clock_addr,
            Some(RpcAccountInfoConfig {
                encoding: Some(UiAccountEncoding::Base64),
                commitment: Some(CommitmentConfig::processed()),
                data_slice: None,
            }),
        )
        .unwrap();

        for ui_account_response in clock_receiver {
            let ui_account = ui_account_response.value;
            let account = ui_account.decode::<Account>().unwrap();
            let clock = deserialize_clock(account.data);
            let blocktime = clock.unix_timestamp;

            if blocktime > latest_blocktime {
                latest_blocktime = blocktime;
                blocktime_sender.send(blocktime).unwrap()
            }
        }
    });
    return blocktime_receiver;
}

fn deserialize_clock(data: Vec<u8>) -> Clock {
    Clock {
        slot: Slot::from_le_bytes(data.as_slice()[0..8].try_into().unwrap()),
        epoch_start_timestamp: UnixTimestamp::from_le_bytes(
            data.as_slice()[8..16].try_into().unwrap(),
        ),
        epoch: Epoch::from_le_bytes(data.as_slice()[16..24].try_into().unwrap()),
        leader_schedule_epoch: Epoch::from_le_bytes(data.as_slice()[24..32].try_into().unwrap()),
        unix_timestamp: UnixTimestamp::from_le_bytes(data.as_slice()[32..40].try_into().unwrap()),
    }
}
