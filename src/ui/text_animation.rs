use bevy::prelude::*;

use crate::models::ui::text::{AnimatingText, TextAnimation, AnimatedText};

pub fn start_animate_text(
    mut cmd: Commands,
    qry: Query<Entity, (Changed<TextAnimation>, With<Text>)>,
  ) {
    for e in qry.iter() {
      cmd.entity(e).insert(AnimatingText { progress: 0.0 });
    }
  }
  
pub fn animate_text(
    mut cmd: Commands,
    mut qry: Query<(Entity, &mut Text, &mut AnimatingText, &TextAnimation)>,
    time: Res<Time>,
  ) {
    for (e, mut txt, mut prog, anim) in qry.iter_mut() {
      prog.progress += anim.animation_speed * time.delta_seconds();
  
      if prog.progress >= 1.0 {
        cmd.entity(e).remove::<AnimatingText>().insert(AnimatedText);
        txt.sections.first_mut().unwrap().value = anim.text.clone();
      } else {
        let l = (prog.progress * anim.text.len() as f32) as usize;
        txt.sections.first_mut().unwrap().value = anim.text[..l].to_owned();
      }
    }
  }