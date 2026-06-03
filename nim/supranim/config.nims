when defined(linux):
  --passL:"/usr/lib/x86_64-linux-gnu/ -levent -levent_pthreads"
  --passC:"-I /usr/include"
elif defined(macosx):
  --passL:"-L /opt/local/lib/ -levent -levent_pthreads"
  --passC:"-I /opt/local/include"
  --passC:"-I /opt/local/include/event2"

