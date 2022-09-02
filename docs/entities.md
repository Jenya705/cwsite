# Rest entities 

## Player
* id - String with length of 36 (uuid)
* name - String with maximum length of 16 and minimum of 3
* role - Bitmask. First 3 bytes reserved for roles other for additional roles

## Roles
* default = 0
* premium = 1
* test_moderator = 2
* moderator = 3
* main_moderator = 4
* developer = 5
* admin = 7

## Additional roles
Yet, no additional roles.

## Application
* id - Id of the player (uuid)
* source - [Source](#source) of his knowledge

## Source
type equals to {} then value is - {}:
* "Player" - player id
* "Website" - website url

## Application page
* applications - array of [Application](#application)

## Application decline reason
* visible_reason - String
* real_reason - String