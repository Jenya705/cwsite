package space.cubicworld.cwsite.repository;

import org.springframework.data.repository.reactive.ReactiveCrudRepository;
import org.springframework.stereotype.Repository;
import reactor.core.publisher.Mono;
import space.cubicworld.cwsite.model.UserModel;

import java.util.UUID;

@Repository
public interface UserRepository extends ReactiveCrudRepository<UserModel, UUID> {

    Mono<UserModel> findByName(String name);

    Mono<UserModel> findByDiscordId(long discordId);

}
