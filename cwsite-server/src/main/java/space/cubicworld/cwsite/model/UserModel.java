package space.cubicworld.cwsite.model;

import lombok.Data;
import org.springframework.data.annotation.Id;
import org.springframework.data.relational.core.mapping.Table;

import java.util.UUID;

@Data
@Table("users")
public class UserModel {

    @Id
    private UUID uuid;

    private String name;

    private long discordId;

}
