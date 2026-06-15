FROM debian:stable-slim AS build

WORKDIR /opt/vlang

RUN apt-get -qq update && \
  apt-get -qy install --no-install-recommends \
    build-essential \
    ca-certificates \
    curl \
    unzip \
    git && \
  {{#build_deps.length}}
  apt-get -y install {{#build_deps}}{{.}} {{/build_deps}} && \
  {{/build_deps.length}}
  rm -rf /var/lib/apt/lists/*

ENV PATH=/opt/vlang:$PATH \
  GIT_SSL_NO_VERIFY=1


# Pin V to the 0.5.1 RELEASE (prebuilt binary) for reproducible builds. We use
# the tagged release rather than building a master commit from source: post-0.5.1
# master carries a large codegen regression — single-element array push (`arr <<
# x`) is 4-7x slower (vlang/v#27468), which slows every V program — so a stable
# tagged release gives a faster and byte-for-byte reproducible compiler. Bump the
# version below to update the compiler.
RUN curl -fsSL https://github.com/vlang/v/releases/download/0.5.1/v_linux.zip -o /tmp/v.zip && \
  unzip -q /tmp/v.zip -d /tmp && rm /tmp/v.zip && \
  cp -a /tmp/v/. /opt/vlang/ && rm -rf /tmp/v

RUN ln -s /opt/vlang/v /usr/bin/v

{{#download}}
  RUN {{{.}}}
{{/download}}

WORKDIR /app

{{#files}}
  COPY '{{source}}' '{{target}}'
{{/files}}

{{#bootstrap}}
  RUN {{{.}}}
{{/bootstrap}}

RUN v {{#build_flags}} {{{.}}} {{/build_flags}} . -o server



FROM debian:stable-slim

WORKDIR /app

RUN apt-get -qq update && \
  apt-get -y install --no-install-recommends curl liburing2 && \
  rm -rf /var/lib/apt/lists/*

# Copy the built binary
COPY --from=build /app /app

HEALTHCHECK --interval=30s --timeout=10s --retries=3 --start-period=40s \
  CMD curl --fail http://0.0.0.0:3000 || exit 1

ENTRYPOINT {{{command}}}
