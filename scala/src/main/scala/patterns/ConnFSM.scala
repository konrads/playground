package patterns

import scala.util.Random

/**
  * Simple Connection that can:
  * - connect
  * - disconnect
  * - run command
  */

class ConnException(msg: String) extends Exception(msg)

trait ConnState {
  def connect(fsm: ConnFSM, user: String, pwd: String): Boolean
  def disconnect(fsm: ConnFSM): Boolean
  def run(fsm: ConnFSM, cmd: String): String
}

class DisconnectedState extends ConnState {
  override def connect(fsm: ConnFSM, user: String, pwd: String): Boolean = {
    // note: rand defined in the package object
    val shouldConn = rand nextBoolean()
    if (shouldConn)
      fsm.state = new ConnectedState
    shouldConn
  }

  override def disconnect(fsm: ConnFSM): Boolean = false

  override def run(fsm: ConnFSM, cmd: String): String = throw new ConnException("Sorry, disconnected")
}

class ConnectedState extends ConnState {
  override def connect(fsm: ConnFSM, user: String, pwd: String): Boolean = false

  override def disconnect(fsm: ConnFSM): Boolean = {
    val shouldDisconn = rand nextBoolean()
    if (shouldDisconn)
      fsm.state = new DisconnectedState
    shouldDisconn
  }

  override def run(fsm: ConnFSM, cmd: String): String = s"cmd: $cmd"
}

class ConnFSM(var state: ConnState = new DisconnectedState) {
  def connect(user: String, pwd: String): Boolean = state connect(this, user, pwd)
  def disconnect(): Boolean = state disconnect(this)
  def run(cmd: String): String = state run(this, cmd)
}
