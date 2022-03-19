FROM rust:1.59.0-buster

# Ignore debconf warning
ENV DEBCONF_NOWARNINGS=yes

#RUN apt-get update \
# && apt-get install -y --no-install-recommends \
# && apt-get install -y vim \
# && apt-get -y clean \
# && rm -rf /var/lib/apt/lists/*

# Enable ll command
RUN sed -i -e "12 s/# //g" /root/.bashrc

WORKDIR /rust
