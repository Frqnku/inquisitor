FROM ubuntu:22.04
RUN apt-get update && apt-get install -y vsftpd
COPY vsftpd.conf /etc/vsftpd.conf
RUN mkdir -p /var/ftp && chmod 777 /var/ftp
EXPOSE 21 20
CMD ["vsftpd", "/etc/vsftpd.conf"]