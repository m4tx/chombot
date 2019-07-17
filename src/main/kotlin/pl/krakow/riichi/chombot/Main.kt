package pl.krakow.riichi.chombot

import discord4j.core.DiscordClientBuilder
import discord4j.core.event.domain.lifecycle.ReadyEvent
import discord4j.core.event.domain.message.MessageCreateEvent
import pl.krakow.riichi.chombot.commands.AkagiInflationRate
import pl.krakow.riichi.chombot.commands.chombo.ChomboCommand
import pl.krakow.riichi.chombot.commands.chombo.SimpleEmbedFormatter
import pl.krakow.riichi.chombot.commands.hand.DrawHandCommand
import reactor.core.publisher.Flux
import reactor.core.publisher.Mono


fun main() {
    val token = System.getenv("CHOMBOT_TOKEN")
    if (token == null) {
        System.err.println("CHOMBOT_TOKEN env variable not set")
        return
    }

    val client = DiscordClientBuilder(token).build()

    client.eventDispatcher.on(ReadyEvent::class.java)
        .subscribe { ready -> println("Logged in as " + ready.self.username) }

    val commandMap = mapOf(
        "chombo" to ChomboCommand(SimpleEmbedFormatter()),
        "hand" to DrawHandCommand(),
        "_inflation" to AkagiInflationRate()
    )

    client.eventDispatcher.on(MessageCreateEvent::class.java)
        .flatMap { event ->
            if (event.message.author.get().isBot) {
                Mono.empty()
            } else {
                Flux.fromIterable(commandMap.entries)
                    .filter { entry -> entry.value.isApplicable(event, entry.key) }
                    .flatMap { entry -> entry.value.execute(event) }
                    .next()
            }
        }
        .subscribe()

    client.login().block()
}
