name := "scala"

version := "1.0"

scalaVersion := "2.11.8"

resolvers += Resolver.sonatypeRepo("releases")

libraryDependencies ++= Seq(
  "com.typesafe.akka"          %% "akka-actor"                % "2.4.17",
  "com.typesafe.akka"          %% "akka-testkit"              % "2.4.17",
  "org.typelevel"              %% "cats"                      % "0.9.0",
  "com.chuusai"                %% "shapeless"                 % "2.3.2",
  "org.scalacheck"             %% "scalacheck"                % "1.13.4",
  "com.github.alexarchambault" %% "scalacheck-shapeless_1.13" % "1.1.3",
  "org.scalaz"                 %% "scalaz-core"               % "7.2.10",
  "org.apache.spark"           %% "spark-core"                % "2.1.0"   % "provided",
  "org.apache.spark"           %% "spark-sql"                 % "2.1.0"   % "provided",
  "org.scalatest"              %% "scalatest"                 % "3.0.0"   % Test,
  "org.specs2"                 %% "specs2-core"               % "4.3.4"   % Test
)
