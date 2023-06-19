FROM paritytech/ci-linux:production

WORKDIR /app/

COPY . ./

EXPOSE 9934
EXPOSE 9945

RUN chmod +x run.sh


ENTRYPOINT ["./node2.sh"]