# 注意
ここにある証明書はオレオレ証明書(self signed)です  
警告され、実際には使うことはできません  

# TLS認証(keyとcertの作成方法)
opensslを使用します  
server.crt, server.key, server.passwordが最終的に必要です  
## 1. 秘密鍵の作成
秘密鍵の生成  
パスワードを聞かれるので、任意のパスワードを決める
```
openssl genrsa -aes128 2048 > server.key
```
## 2. CSRの作成(サーバ証明書への署名)
CSRの作成  
パスワードを聞かれるので前に決めたパスワードを入力
```
openssl req -new -key server.key > server.csr
```
組織名などの入力
```
Enter pass phrase ofr server.key: パスフレーズ　　　←（任意）
You are about to be asked to enter information that will be incorporated 
into your certificate request.
What you are about to enter is what is called a Distinguished Name or a DN.
There are quite a few fields but you can leave some blank
For some fields there will be a default value,
fi you enter '.', the field will be left blank.
Country Name (2 letter code) [AU]: JP　　　←（国名）
State or Province Name (full name) [Some-state]: Tokyo　　　←（都道府県名？）
Locality Name (eg, city) []: Shibadaimon, Minato-ku　　　←（市区町村名？）
Organization Name (eg, company) [Internet Widgits Pty Ltd]: Example Inc.　　　←（会社名）
Organizational Unit Name (eg, section) []: Example Section　　　←（？）
Common Name (eg, YOUR name) []: example.com　　　←（任意）
Email Address []: hoge@fuga.jp　　　←（メールアドレス）


Pleaseenter thee following 'extra' attributes to be sent with your certificate request
A challenge password[]: パスワード　　　←（任意）
An optional company name []: hogehoge　
```
## 3. デジタル証明書(crt)の作成
```
openssl x509 -in server.csr -days 365000 -req -signkey server.key > server.crt
```
## 4. パスワード
設定したパスワードをechoでファイルに書き込む  
```
構文  
echo {パスワード} > server.password
```

```
echo pass_word > server.password
```
