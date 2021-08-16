use sqlx::mysql::{MySqlPool,MySqlConnection};
use sqlx::prelude::*;
use std::env;
use std::fs;//引入rust 标准库中的fs模块
#[async_std::main]
async fn main()-> anyhow::Result<()> {


    //单个连接
    let mut pool = MySqlConnection::connect(&env::var("DATABASE_URL")?).await?;
    //block_info();
    let sql="INSERT INTO Head VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17,$18,$19,$20,$21)";
    let count=sqlx::query(sql).bind(hash).bind(parent_hash).bind(sha3_uncles).bind(miner).bind(state_root).bind(transactions_root)
        .bind(receipts_root).bind(number).bind(gas_used).bind(gas_limit).bind(base_fee_per_gas).bind(extra_data).bind(logs_bloom).bind(timestamp)
        .bind(difficulty).bind(total_difficulty).bind(seal_fields).bind(uncles).bind(size).bind(mix_hash).bind(nonce).execute(&mut pool).await?;
    println!("{}",count);
/*
//SELECT block_number, block_hash, from_addr, to_addr FROM body where block_number = 1
    let sql = "SELECT * FROM body where block_number = 1";
    let mut cursor = sqlx::query(sql).fetch(&mut pool);
    while let Some(row) = cursor.next().await? {

        let block_number: String = row.get(0);
        let block_hash: String = row.get(1);
        let from_addr: String = row.get(2);
        let to_addr: String = row.get(3);

        println!("block_number: {}, block_hash: {}，from_addr: {}, to_addr: {}",block_number,block_hash,from_addr,to_addr);

      }
*/
    Ok(())
}
