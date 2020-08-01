package com.packt.chapter1

import akka.actor.Actor
import akka.actor.Props
import akka.actor.ActorSystem

class SummingActor extends Actor {
    var sum = 0
    
    override def receive: Receive = {
        case x: Int => sum = sum + x
            println(s"my state as sum is $sum")

        case _ => println("Idon't know what are you talking about")
    }
}

object BehaviorAndState extends App {
  val actorSystem = ActorSystem("HelloAkka")
  val actor = actorSystem.actorOf(Props[SummingActor])
  println(actorSystem)
  println(actor.path)
  while (true) {
    Thread.sleep(1)
    actor ! 1
  }

}
