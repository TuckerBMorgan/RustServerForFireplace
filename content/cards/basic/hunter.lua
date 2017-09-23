minion
@@
create_minion_function**
    m = Minion.new("hunter", give_uid, 0, "basic", 0, 30, "hunter");
    m:add_tag("Hero")
    result = m

@@
on_hp_change_function**
    mh = Rune.new_modify_hero_health(target_uid, amount)
    
    result = {}
    result[1]=RuneTypeEnum.new_modify_hero_health(mh)

@@
generate_options_function**
