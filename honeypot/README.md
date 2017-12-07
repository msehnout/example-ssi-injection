# How to use:

* Build docker image:
```
docker build -t ssi-apache-honeypot
```

* Run it:
```
docker run -dit --name apache-honeypot -p 127.0.0.1:8080:80 -v $PWD/html:/usr/local/apache2/htdocs/ ssi-apache-honeypot
```
