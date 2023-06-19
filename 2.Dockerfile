FROM paritytech/ci-linux:production

WORKDIR /app/

COPY . ./

EXPOSE 9933
EXPOSE 9944

RUN chmod +x run.sh


ENTRYPOINT ["./node2.sh"]