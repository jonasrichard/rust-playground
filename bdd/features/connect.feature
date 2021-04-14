Feature: Client connect

    Scenario Outline: Client connect with default parameters
        Given A connection
        When it connects
        Then I am ok
