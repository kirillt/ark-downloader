This is a thin wrapper of youtube-dl with REST API.

You just send your link to it and after some time the content appears in folder where you started the server:
```
curl 127.0.0.1:1337/submit -d https://www.youtube.com/watch?v=N3KsbS60pEc
```

I have launched this on my server where I also spawned Syncthing.\
So I send links via Termux from my phone, the server downloads them and\
when I come home and open my laptop (or my phone connects to Wi-Fi)\
videos download there pretty quick.

Probably sometime I will have time to make a UI for it.
