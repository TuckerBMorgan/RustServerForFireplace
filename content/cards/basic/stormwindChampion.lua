minion
@@
create_minion_function**
    m = Minion.new("stormwindChampion", game_state_data:get_uid(), 7, "basic", 6, 6, "Stormwind Champion");
    m:add_tag("Aura")
    result = m
@@
filter_function**
    count = 0
    result = {}
    team1 = enchanter:get_team()
    index = 1
    while count < minions_len do
        min = minions:get(count)
        team2 = min:get_team()
        if team1 == team2 then
            rets[index] = min  
            index = index + 1
        end
    end