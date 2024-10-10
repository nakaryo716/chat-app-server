# Zircon
## Overview
**Zircon** は、Rustで書かれた軽量かつシンプルなオープンソースのチャット機能を提供するWeb APIです。  
WebSocketを使用し、リアルタイムチャットを実現します。最小限の機能で動作し、拡張が容易です。  
## Features
- 軽量で高速なチャット機能
- WebSocketを使用したリアルタイム通信
- シンプルなAPI設計
- JWTを使用した認証
## Getting Started
### Prerequisites
以下のソフトウェアが必要です:
- [Docker](https://www.docker.com/)
### Installation
1. リポジトリをクローンします:
    ```bash
    git clone https://github.com/nakaryo716/zircon.git
    cd zircon
    ```
2. Dockerコンテナを立ち上げます:  
    ```bash
    docker compose up
    ```
3. APIが起動します。デフォルトではlocalhost:8080でアクセス可能です。
## Usage
APIを使用するにはユーザー認証が必要です  
ユーザー登録及びユーザー認証を行った後、チャットルームの作成やチャットを行うことができます
## API Endpoints
### ユーザー登録
Method: ```POST```  
URL: ```http://localhost:8080/user```  
Request Body:
```json
{
    "userName": "your user name",
    "userMail": "yourmail@mail.com",
    "userPass": "youruserpass"
}
```
### ログイン
Method: ```POST```  
URL: ```http://localhost:8080/login```  
Request Body:
```json
{
    "userMail": "yourmail@mail.com",
    "userPass": "youruserpass"
}
```
CookieにJWTが保存される
### ユーザー情報取得
Method: ```GET```  
URL: ```http://localhost:8080/user```  
Auth: JWTが有効である必要がある
### ユーザー削除
Method: ```DELETE```  
URL: ```http://localhost:8080/user```   
Auth: JWTが有効である必要がある  
Request Body:
```json
{
    "userMail": "yourmail@mail.com",
    "userPass": "youruserpass"
}
```
### チャットルーム作成
Method: ```POST```  
URL: ```http://localhost:8080/room```  
Auth: JWTが有効である必要がある  
Request Body:
```json
{
    "roomName": "room name",
}
```
### 全てのチャットルーム情報取得
Method: ```GET```  
URL: ```http://localhost:8080/room```  
Auth: JWTが有効である必要がある  
### 特定のチャットルーム情報取得
Method: ```GET```  
URL: ```http://localhost:8080/room/:id```  
Auth: JWTが有効である必要がある  
### チャットルームの削除
Method: ```DELETE```  
URL: ```http://localhost:8080/room/:id```  
Auth: JWTが有効である必要がある  
### チャット参加(WebSocket)
Method: ```GET```  
URL: ```ws://localhost:8080/chat/:id```  
Auth: JWTが有効である必要がある  
## License
This project is licensed under the MIT License - see the LICENSE file for details.

