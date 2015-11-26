package langfeatures

import akka.actor._
import akka.pattern.ask
import akka.util.Timeout

import scala.concurrent.Await
import scala.concurrent.duration._

class HelloActor extends Actor {
  def receive = {
    case "hello" => println(s"hello back to $sender")
    case x => println(s"unexpected greeting $x -- $sender")
  }
}

class HelloFromActor(val name: String) extends Actor {
  def receive = {
    case "hello" => {
      val msg = s"actor $name: hello back from to $sender"
      println(msg)
    }
    case x => {
      val msg = s"actor $name: unexpected greeting $x -- $sender"
      println(msg)
      Thread sleep 3000
      sender ! msg
    }
  }
}

object HelloActor {
  def main(args: Array[String]) {
    implicit val ___some_sort_of_timeout = Timeout(3 seconds)
    val system = ActorSystem("HelloSystem")
    implicit val inbox: Inbox = Inbox.create(system)
    val helloActor = system.actorOf(Props(classOf[HelloFromActor], "johny"), name = "hellofromactor")
    helloActor ! "hello"
    val f = (helloActor ? "balls").mapTo[String]
    val v = Await.result(f, ___some_sort_of_timeout.duration)  // timeout needed again?!?
    println(s"from main: $v")
  }
}
