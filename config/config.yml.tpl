server:
  host: 0.0.0.0
  port: 9870

Cache:
  host: 127.0.0.1
  port: 6379
  Db: 0
  user: aa
  pass_word:  aa

Db:
  host: 127.0.0.1
  port: 3306
  pg_port: 5432
  user: root
  pg_user: aa
  password: aa
  db: blog

Qiniu:
  accessKey: aa
  secretKey: aa
  bucket: aa
  host: https://cdn.xx.com/

GitHub:
  client_id: aa,
  client_secret: aa,
  redirect_uri: https://www.bytealien.com/api/user/github/oauth

AIArticle:
  enabled: false
  # robfig/cron v1 format: second minute hour day month weekday
  spec: "0 37 6 * * ?"
  api_key:
  api_base: https://api.openai.com/v1
  model: gpt-4o-mini
  temperature: 0.7
  timeout_seconds: 90
  category_id: 2
  user_id: 1

Mail:
  smtp_host:  smtp.gmail.com
  smtp_port:  587
  smtp_username: xxx@gmail.com
  smtp_username: xxxx
  max_client: 100

AliPay:
  # 应用私钥
  private_key: xx
  # 支付宝公钥
  public_key: xx
  app_id: xx

 WechatPay:
   app_iD:
   app_secret:
