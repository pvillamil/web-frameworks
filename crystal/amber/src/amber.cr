require "../config/application"

if ARGV.size > 0 && ARGV[0] == "--start-amber"
  Amber::Server.start
else
  System.cpu_count.times do
    Process.new(PROGRAM_NAME, ["--start-amber"])
  end
end

sleep
