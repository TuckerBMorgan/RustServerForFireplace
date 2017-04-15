minion
@@
create_minion_function**
    m = Minion.new("dragonlingMechanic", give_uid, 0, "basic", 2, 4, "Dragonling Mechanic");
    m:add_tag("BattleCry")
    result = m
@@
battle_cry_function**
    cc = Rune.new_create_card("dragonlingMechanic", give_uid, controller_uid);
    rm = Rune.new_report_minion_to_client();
    sm = Rune.new_summon_minion(give_uid, controller_uid, index + 1);