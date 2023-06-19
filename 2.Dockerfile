FROM paritytech/ci-linux:production

WORKDIR /app/

COPY . ./

EXPOSE 9934
EXPOSE 9945

RUN chmod +x node2.sh


ENTRYPOINT ["./node2.sh"]