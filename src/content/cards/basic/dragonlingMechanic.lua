minion
@@
create_minion_function**
    m = Minion.new("dragonlingMechanic", give_uid, 0, "basic", 2, 4, "Dragonling Mechanic");
    m:add_tag("BattleCry")
    result = m
@@
battle_cry_function**
    cc = Rune.new_create_card("basic/mechanicalDragonling", give_uid, controller_uid);
    sm = Rune.new_summon_minion(give_uid, controller_uid, index + 1);
    
    ecc = RuneTypeEnum.new_create_card(cc);
    ecm = RuneTypeEnum.new_summon_minion(sm);

    result = {}    
    result[1] = ecc;
    result[2] = ecm;