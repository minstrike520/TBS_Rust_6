pub mod manager;

use crate::game::characters;

pub trait IEffect {
    fn new() -> Self where Self: Sized;
    fn effect_data(&mut self) -> &mut EffectData;
    fn affect(&self, target: &mut characters::Character) -> ();
}

pub trait IConsumable {
    fn consume(&mut self) -> u32;
}

pub trait ITunerEffect: IEffect {
    fn addition_percentage(&self) -> i32;
    fn addend(&self) -> i32;
}
pub trait IHealthDecreaseEffect: IEffect {}
pub trait IHealthRegenEffect: IEffect {}

struct A;
impl ITunerEffect for A {

}

pub enum Effect {
    //RegularEffect(RegularEffect),
}
#[derive()]
pub enum RegularEffect<T, HD, HR> 
where 
    T: ITunerEffect, 
    HD: IHealthDecreaseEffect,
    HR: IHealthRegenEffect {
    Tuner(T),
    HealthDecrease(HD),
    HealthRegen(HR),
}
pub enum TurnSession {
    End,
}
pub struct EffectCounter {
    pub counting_every_turn: bool,
    pub initial: u32,
    pub remain: u32
}
impl EffectCounter {
    pub fn consume (&mut self) -> u32 {
        self.remain -= 1;
        self.remain
    }
}

pub struct EffectData {
    pub name: String, 
    pub r#type: EffectType,
    pub counting: Option<EffectCounter>,
    pub affect_timing: TurnSession,
}

/*
pub struct Effect {
    pub name: String, 
    pub origin: EffectOriginData,
    pub r#type: EffectType,
    pub counting_type: TurnCountingType,
    pub affect: Box<dyn FnMut(&mut chars::Character) -> ()>,
}
impl Effect {
    pub fn new(
        name: String, 
        origin: EffectOriginData,
        r#type: EffectType,
        counting_type: TurnCountingType,
        affect: impl FnMut(&mut chars::Character) -> () + 'static 
    ) -> Self {
        Self { name, origin, r#type, counting_type, affect: Box::new(affect) }
    }
    pub fn affect(&mut self, target: &mut chars::Character) {
        (self.affect)(target);
    }
}*/


/*    (from chars.rs)

    pub fn add_effect<T>(&mut self, effect: T) where T: effects::Effect + 'static {
        self.effects.push(Box::from(effect));
    }
    pub fn every_turn_ends(&mut self) {
        
        /*
            if effect.counting_type == effects::TurnCountingType::AnyTurn(effects::TurnTiming::End) {
                effect.affect(self);
            }
        */
    }


 */

 /*pub fn boost (caster: &mut characters::Character) {
    struct Boosted(EffectData); impl Effect for Boosted {
        fn affect(&self, target: &mut characters::Character) -> () {
            target.attributes.atk.add(30, 10);
        }
        fn effect_data(&mut self) -> &mut EffectData { &mut self.0 }
        fn new() -> Self { Self(EffectData{
            name: "Boosted".to_string(),
            r#type: EffectType::Tuner,
            counting: Some(EffectCounter{counting_every_turn: true, initial: 3, remain: 3}),
            affect_timing: TurnSession::End,
        })}
    }
    //caster.effects.push(Box::from(Boosted::new()))
} */