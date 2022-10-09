FROM alpine:3.14

ENV TZ=Asia/Shanghai \
    GO111MODULE=on \
    CGO_ENABLED=0 \
    GOOS=linux \
    GOARCH=amd64 \
    PROGRAM_ENV=pro

WORKDIR /src/build

# 复制构建应用程序所需的代码
COPY ./build .


EXPOSE 1025

CMD ["./blog-backend"]
