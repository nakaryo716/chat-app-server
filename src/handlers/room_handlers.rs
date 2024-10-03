// ルームを作るハンドラ
// json形式でroom nameを取得
// トークンを確認しユーザー情報取得
// >>> 以上の2つのデータをもとに関数呼び出しで作成 <<< サービスに移譲
// roomのidを取得する
// statuscode okとjson形式でidを渡す

use crate::{database::rooms_db::RoomDb, models::rooms_model::CreateRoom};
use axum::{extract::State, response::IntoResponse, Json};
use http::StatusCode;

async fn create_room_handler(
    State(room_db): State<RoomDb>,
    Json(payload): Json<CreateRoom>,
) -> Result<impl IntoResponse, StatusCode> {
    // let a = create_room(&room_db, payload);
    Ok(())
}
