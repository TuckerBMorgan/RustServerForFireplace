minion
@@
create_minion_function**
    m = Minion.new("stormwindChampion", give_uid, 7, "basic", 6, 6, "Stormwind Champion");
    m:add_tag("Aura")
    result = m
@@
filter_function**
    count = 1
    result = {}
    team1 = enchanter:get_team()
    index = 1
    while count <= minions["n"] do
        min = minions[count]
        team2 = min:get_team()
        if team1 == team2 then
            result[index] = min
            index = index + 1
        end
        count = count + 1
    end
@@
apply_aura**
    sh = Rune.new_set_health(getter:get_uid(), getter:get_total_health() + 1)

    mh = Rune.new_modify_health(getter:get_uid(), 1)
    
    sa = Rune.new_set_attack(getter:get_uid(), getter:get_total_attack() + 1)
    
    ma = Rune.new_modify_attack(getter:get_uid(), 1)
    
    result = {}
    result[1] = RuneTypeEnum.new_set_health(sh)
    result[2] = RuneTypeEnum.new_modify_health(mh)
    result[3] = RuneTypeEnum.new_set_attack(sa)
    result[4] = RuneTypeEnum.new_modify_attack(ma)

@@
remove_aura**
    
    sh = Rune.new_set_health(loser:get_uid(), loser:get_total_health() - 1)
    sa = Rune.new_set_attack(loser:get_uid(), loser:get_total_attack() - 1)

    result = {}
    result[1] = RuneTypeEnum.new_set_health(sh)
    result[2] = RuneTypeEnum.new_set_attack(sa)