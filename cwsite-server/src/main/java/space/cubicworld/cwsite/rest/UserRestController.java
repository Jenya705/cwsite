package space.cubicworld.cwsite.rest;

import lombok.RequiredArgsConstructor;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RestController;
import reactor.core.publisher.Mono;
import space.cubicworld.cwsite.model.UserModel;
import space.cubicworld.cwsite.repository.UserRepository;

import java.util.UUID;

@RestController("/user")
@RequiredArgsConstructor(onConstructor_ = @Autowired)
public class UserRestController {

    private final UserRepository repository;

    @GetMapping("/id/{id}")
    public Mono<UserModel> findUserById(UUID id) {
        return repository.findById(id);
    }

    @GetMapping("/discord/{discordId}")
    public Mono<UserModel> findUserByDiscord(long discordId) {
        return repository.findByDiscordId(discordId);
    }

    @GetMapping("/name/{name}")
    public Mono<UserModel> findUserByName(String name) {
        return repository.findByName(name);
    }

}
