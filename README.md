# zipcode-japan

![https://i.imgur.com/jhDECiE.gif](https://i.imgur.com/jhDECiE.gif)

(株)日本郵政の[郵便番号データダウンロード](https://www.post.japanpost.jp/zipcode/download.html)より取得した日本全国の郵便番号データを JSON 形式に加工し、
GitHub Pages 上で公開しています。

## Usage

郵便番号から住所を求めることができます。

```bash
curl https://yuukitoriyama.github.io/zipcode-japan/v0.3/602/8491.json
```

```terminal
{
  "zipCode": "6028491",
  "pref": "京都府",
  "prefKana": "キョウトフ",
  "city": "京都市上京区",
  "cityKana": "キョウトシカミギョウク",
  "town": "西社町",
  "townKana": "ニシヤシロチョウ"
}
```

### Breaking change

#### API エンドポイントが変更になっています(2022 年 10 月 21 日更新)

```text
旧URL: https://yuukitoriyama.github.io/zipcode-japan/xxx/xxxx.json
新URL: https://yuukitoriyama.github.io/zipcode-japan/v0.3/xxx/xxxx.json
```

## Author

YUUKIToriyama([@CoconMap](https://twitter.com/CoconMap))
