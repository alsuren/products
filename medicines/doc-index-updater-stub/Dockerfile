FROM perl:5.26

WORKDIR /usr/src/stub
COPY . .

RUN cpanm Carton
run carton install

EXPOSE 8080

ENV STUB_USERNAME username
ENV STUB_PASSWORD password

CMD ["carton", "exec", "plackup", "-s", "Starman", "-p", "8080", "--workers", "1", "-a", "stub.pl"]
