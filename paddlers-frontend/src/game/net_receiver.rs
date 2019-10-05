use crate::prelude::*;
use crate::game::{
    units::worker_factory::create_worker_entities,
    units::workers::Worker,
    components::*,
    };
use crate::net::{
    NetMsg, 
};

use specs::prelude::*;
use super::*;

impl Game<'_,'_> {
    pub (super) fn update_net(&mut self) -> PadlResult<()> {
        use std::sync::mpsc::TryRecvError;
        match self.net.as_ref().unwrap().try_recv() {
            Ok(msg) => {
                // println!("Received Network data!");
                match msg {
                    NetMsg::Error(msg) => {
                        println!("Network Error: {}", msg);
                    }
                    NetMsg::Attacks(response) => {
                        if let Some(data) = response.data {
                            for atk in data.village.attacks {
                                atk.create_entities(&mut self.world, self.unit_len.unwrap())?;
                            }
                        }
                        else {
                            println!("No data returned");
                        }
                    }
                    NetMsg::Buildings(response) => {
                        if let Some(data) = response.data {
                            data.create_entities(self);
                        }
                        else {
                            println!("No buildings available");
                        }
                    }
                    NetMsg::Map(response, min, max) => {
                        if let Some(data) = response.data {
                            let streams = data.map.streams.iter()
                                .map(
                                    |s| {
                                        s.control_points
                                            .chunks(2)
                                            .map(|slice| (slice[0] as f32, slice[1] as f32))
                                            .collect()
                                    }
                                )
                                .collect();
                            let villages = data.map.villages.into_iter().map(VillageMetaInfo::from).collect();
                            let (map, world) = (self.map.as_mut(), &mut self.world);
                            map.map(|map| map.add_segment(world, streams, villages, min, max));
                        }
                        else {
                            println!("No map data available");
                        }
                    },
                    NetMsg::Resources(response) => {
                        if let Some(data) = response.data {
                            self.resources.update(data);
                        }
                        else {
                            println!("No resources available");
                        }
                    }
                    NetMsg::Workers(response) => {
                        let now = self.world.read_resource::<Now>().0;
                        let results = create_worker_entities(&response, &mut self.world, now);
                        let mut q = self.world.write_resource::<ErrorQueue>();
                        for res in results.into_iter() {
                            if let Err(e) = res {
                                q.push(e);
                            }
                        }
                    }
                    NetMsg::UpdateWorkerTasks(unit) => {
                        let entity = self.worker_entity_by_net_id(unit.id.parse().unwrap())?;
                        let workers = &mut self.world.write_storage::<Worker>();
                        let worker = workers.get_mut(entity).unwrap();
                        worker.tasks.clear();

                        let net = self.world.read_storage::<NetObj>();
                        let ent = self.world.entities();
                        for task in unit.tasks {
                            match task.create(&net, &ent) {
                                Ok(task) => worker.tasks.push_back(task),
                                Err(e) => {
                                    match e.err {
                                        PadlErrorCode::UnknownNetObj(_) => {
                                            // NOP: Best to ignore and move on with other tasks
                                        },
                                        _ => { return Err(e); }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            Err(TryRecvError::Disconnected) => { return PadlErrorCode::NoNetwork.usr(); },
            Err(TryRecvError::Empty) => {},
        }
        Ok(())
    }
}