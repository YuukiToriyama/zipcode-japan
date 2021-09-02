# zipcode-japan

(株)日本郵政の[郵便番号データダウンロード](https://www.post.japanpost.jp/zipcode/download.html)より取得した日本全国の郵便番号データをJSON形式に加工し、
GitHub Pages上で公開しています。

## Usage

郵便番号から住所を求めることができます。

```bash
curl https://yuukitoriyama.github.io/zipcode-japan/602/8491.json
```
```terminal
{
  "zipCode": "6028491",
  "pref": "京都府",
  "prefKana": "ｷｮｳﾄﾌ",
  "city": "京都市上京区",
  "cityKana": "ｷｮｳﾄｼｶﾐｷﾞｮｳｸ",
  "town": "西社町",
  "townKana": "ﾆｼﾔｼﾛﾁｮｳ"
}
```

## Author
YUUKIToriyama([@CoconMap](https://twitter.com/CoconMap))
