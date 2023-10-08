use crate::game::characters::{attribute::{self, ITunable}, effects::tuners};

pub fn test_all() {
    attribute_tuning_test();
}

fn attribute_tuning_test() {
    use attribute::IAttribute;
    let mut atk = attribute::Attribute::new(50);
    let a = tuners::Tuner{addition_percentage: 30, addend: 10, time_left: Some(3)};
    let b = tuners::Tuner{addition_percentage: -10, addend: 0, time_left: Some(2)};
    let c = tuners::Tuner{addition_percentage: 0, addend: 50, time_left: None};
    atk.add_tuner(a);
    atk.add_tuner(b);
    atk.add_tuner(c);


    atk.evaluate_all();
    assert_eq!(atk.get(), 120);
    atk.evaluate_all();
    assert_eq!(atk.get(), 120);
    atk.evaluate_all();
    assert_eq!(atk.get(), 125);
    atk.evaluate_all();
    assert_eq!(atk.get(), 100);    
}
/*假設有 A 效果 B效果 C 效果 
A 3回合 B 2 回合 C 常數 
都是末期計算，對攻擊力造成影響
A: 30% + 10
B: -10% + 0
C: 0% + 50 
第一回合末的計算:
... iATK&20%+60 
嘗試消耗一次A
A 還沒用完，勝2次 
嘗試消耗一次B
B還沒用完，剩1次
嘗試消耗一次C
C還沒用完，是常數 
嘗試消耗一次A
A還沒用完，剩1次
嘗試消耗一次B
B還沒用完，剩0次
嘗試消耗一次C
C還沒用完，是常數
第二回合末的計算:
... iATK&20%+60 
沉默的利刃 — Today at 11:11 AM
嘗試消耗一次A
A還沒用完，剩0次
嘗試消耗一次B
B用完了
移除B，並且不計算
嘗試消耗一次C
C還沒用完，是常數
第三回合末的計算:
... iATK&30%+60 */