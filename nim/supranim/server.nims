when defined(linux):
  let evCFlags = gorge("pkg-config --cflags libevent libevent_pthreads").strip
  let evLFlags = gorge("pkg-config --libs --static libevent libevent_pthreads").strip
  switch("passC", evCFlags)
  switch("passL", evLFlags)
elif defined(macosx):
  --passL:"-L /opt/local/lib/ -levent -levent_pthreads"
  --passC:"-I /opt/local/include"
  --passC:"-I /opt/local/include/event2"
