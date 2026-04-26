Feature: Animal feature

Scenario: If we feed a hungry cat it will no longer be hungry
Given a hungry cat
When I feed the cat
Then the cat is happy

Scenario: If we feed a satiated cat it will become curious
Given a satiated cat
When I feed the cat
Then the cat is curious

Scenario: If we pet a hungry cat it will become annoyed
Given a hungry cat
When I pet the cat
Then the cat is annoyed

Scenario: If we feed a hungry cat with tuna it will be happy
Given a hungry cat
    And a can of tuna
When I feed the cat
Then the cat is happy

Scenario: If we feed a hungry cat with beans it will be annoyed
Given a hungry cat
    And a can of beans
When I feed the cat
Then the cat is annoyed