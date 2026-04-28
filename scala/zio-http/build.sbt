val ZioHttpVersion = "3.11.0"

name := "server"

scalaVersion := "3.8.3"

lazy val root = (project in file("."))
  .settings(
    libraryDependencies ++= Seq(
      "dev.zio" %% "zio-http" % ZioHttpVersion
    )
  )
  .enablePlugins(JavaAppPackaging)
