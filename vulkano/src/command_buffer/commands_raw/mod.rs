// Copyright (c) 2016 The vulkano developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

//! All the commands used in the internals of vulkano.
//!
//! This module only contains the base commands that have direct equivalents in the Vulkan API.

pub use self::begin_render_pass::CmdBeginRenderPass;
pub use self::bind_index_buffer::CmdBindIndexBuffer;
pub use self::bind_descriptor_sets::{CmdBindDescriptorSets, CmdBindDescriptorSetsError};
pub use self::bind_pipeline::{CmdBindPipeline, CmdBindPipelineSys};
pub use self::bind_vertex_buffers::{CmdBindVertexBuffers, CmdBindVertexBuffersHash};
pub use self::blit_image::{CmdBlitImage, CmdBlitImageError};
pub use self::clear_attachments::CmdClearAttachments;
pub use self::copy_buffer::{CmdCopyBuffer, CmdCopyBufferError};
pub use self::copy_buffer_to_image::{CmdCopyBufferToImage, CmdCopyBufferToImageError};
pub use self::copy_image::{CmdCopyImage, CmdCopyImageError};
pub use self::dispatch_raw::{CmdDispatchRaw, CmdDispatchRawError};
pub use self::draw_indexed_raw::CmdDrawIndexedRaw;
pub use self::draw_indirect_raw::CmdDrawIndirectRaw;
pub use self::draw_raw::CmdDrawRaw;
pub use self::end_render_pass::CmdEndRenderPass;
pub use self::execute::CmdExecuteCommands;
pub use self::fill_buffer::{CmdFillBuffer, CmdFillBufferError};
pub use self::next_subpass::CmdNextSubpass;
pub use self::pipeline_barrier::CmdPipelineBarrier;
pub use self::push_constants::{CmdPushConstants, CmdPushConstantsError};
pub use self::resolve_image::{CmdResolveImage, CmdResolveImageError};
pub use self::set_event::CmdSetEvent;
pub use self::set_state::{CmdSetState};
pub use self::update_buffer::{CmdUpdateBuffer, CmdUpdateBufferError};

mod begin_render_pass;
mod bind_descriptor_sets;
mod bind_index_buffer;
mod bind_pipeline;
mod bind_vertex_buffers;
mod blit_image;
mod clear_attachments;
mod copy_buffer;
mod copy_buffer_to_image;
mod copy_image;
mod dispatch_raw;
mod draw_indexed_raw;
mod draw_indirect_raw;
mod draw_raw;
mod end_render_pass;
mod execute;
mod fill_buffer;
mod next_subpass;
mod pipeline_barrier;
mod push_constants;
mod resolve_image;
mod set_event;
mod set_state;
mod update_buffer;
