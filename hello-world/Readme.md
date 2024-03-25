# hello-world
This repo will contains POC with simple reverse-proxy configuration

# Yaml structure:
```yaml
routes:
    - route1:
        backend: backend1
    - route2:
        backend: backend2
        add_header:
            - SomeCors: '*'
        proxy_headers:
            - header1: value1
            - header2: value2

backend:
    - backend1:
        host: 1.1.1.1:80
    - backend2:
        host: 1.1.2.2:80
```
