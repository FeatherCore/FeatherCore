## 2026-06-14 A2DP upstream AVDTP session flow closeout

A2DP closeout added a gate for an upstream-shaped AVDTP/MediaTransport session
flow.  The bridge now validates endpoint select, set configuration,
open/start, suspend/close, MediaTransport binding, A2DP resume/cancel id
cleanup, and final transport detach.

Verification artifacts:

- `FeatherCore/build/logs/build-bt1-a2dp-upstream-session-flow.log`
- `FeatherCore/build/logs/build-bt2-a2dp-upstream-session-flow.log`
- `FeatherCore/build/logs/run-a2dp-upstream-session-flow.log`
- `FeatherCore/build/bt-hwsim-usecases-a2dp-upstream-session-flow/run-results.json`

Validated hwsim case:

- `bluez-a2dp-upstream-convergence-closeout`: PASS for bt1/source and
  bt2/sink.
- Source and sink both emit `bluez-daemon: a2dp upstream-handler-bridge-surface-wrapper ... upstream-a2dp-session=select:1,set-config:1,open-start:1,suspend-close:1,total:4 ... final-ok=1`.
- The validator now rejects an A2DP bridge that lacks upstream-shaped
  AVDTP/MediaTransport session flow evidence.

Current boundary:

- AVDTP/MediaTransport session flow is represented by NuttX-side
  upstream-shaped mirror logic.
- The next A2DP convergence target remains replacing mirror flow code with
  directly imported upstream AVDTP/A2DP session callbacks.

## 2026-06-14 A2DP upstream media adapter lifecycle closeout

A2DP closeout added a gate for upstream-shaped `media.c` media adapter
lifecycle.  The bridge now validates adapter probe/register, timestamping
capability capture, endpoint/player feature aggregation, feature update after
endpoint removal, and adapter remove cleanup of apps/endpoints/players.

Verification artifacts:

- `FeatherCore/build/logs/build-bt1-a2dp-upstream-media-adapter.log`
- `FeatherCore/build/logs/build-bt2-a2dp-upstream-media-adapter.log`
- `FeatherCore/build/logs/run-a2dp-upstream-media-adapter.log`
- `FeatherCore/build/bt-hwsim-usecases-a2dp-upstream-media-adapter/run-results.json`

Validated hwsim case:

- `bluez-a2dp-upstream-convergence-closeout`: PASS for bt1/source and
  bt2/sink.
- Source and sink both emit `bluez-daemon: a2dp upstream-handler-bridge-surface-wrapper ... upstream-media-adapter=probe:1,features:1,remove:1,total:3 ... final-ok=1`.
- The validator now rejects an A2DP bridge that lacks upstream-shaped media
  adapter probe/features/remove lifecycle evidence.

Current boundary:

- Media adapter lifecycle is represented by NuttX-side upstream-shaped mirror
  logic.
- The next A2DP convergence target remains replacing mirror code with directly
  imported upstream media adapter probe/remove/update-features behavior.

## 2026-06-14 A2DP upstream local player lifecycle closeout

A2DP closeout added a gate for upstream-shaped `media.c` local player
lifecycle.  The bridge now validates RegisterPlayer object creation, property
watch/seek watch ownership, track/settings/status updates, and
UnregisterPlayer cleanup.

Verification artifacts:

- `FeatherCore/build/logs/build-bt1-a2dp-upstream-local-player.log`
- `FeatherCore/build/logs/build-bt2-a2dp-upstream-local-player.log`
- `FeatherCore/build/logs/run-a2dp-upstream-local-player.log`
- `FeatherCore/build/bt-hwsim-usecases-a2dp-upstream-local-player/run-results.json`

Validated hwsim case:

- `bluez-a2dp-upstream-convergence-closeout`: PASS for bt1/source and
  bt2/sink.
- Source and sink both emit `bluez-daemon: a2dp upstream-handler-bridge-surface-wrapper ... upstream-local-player=register:1,properties:1,unregister:1,total:3 ... final-ok=1`.
- The validator now rejects an A2DP bridge that lacks upstream-shaped local
  player register/properties/unregister lifecycle evidence.

Current boundary:

- Local player lifecycle is represented by NuttX-side upstream-shaped mirror
  logic.
- The next A2DP convergence target remains replacing mirror code with directly
  imported upstream local player creation/properties/cleanup behavior.

## 2026-06-14 A2DP upstream media app lifecycle closeout

A2DP closeout added a gate for upstream-shaped `media.c` application
lifecycle.  The bridge now validates Media1 RegisterApplication app creation,
proxy endpoint/player ownership, UnregisterApplication cleanup, and client
disconnect cleanup of app/endpoints/players/proxies.

Verification artifacts:

- `FeatherCore/build/logs/build-bt1-a2dp-upstream-media-app.log`
- `FeatherCore/build/logs/build-bt2-a2dp-upstream-media-app.log`
- `FeatherCore/build/logs/run-a2dp-upstream-media-app.log`
- `FeatherCore/build/bt-hwsim-usecases-a2dp-upstream-media-app/run-results.json`

Validated hwsim case:

- `bluez-a2dp-upstream-convergence-closeout`: PASS for bt1/source and
  bt2/sink.
- Source and sink both emit `bluez-daemon: a2dp upstream-handler-bridge-surface-wrapper ... upstream-media-app=register:1,unregister:1,disconnect:1,total:3 ... final-ok=1`.
- The validator now rejects an A2DP bridge that lacks upstream-shaped media app
  register/unregister/disconnect lifecycle evidence.

Current boundary:

- Media app lifecycle is represented by NuttX-side upstream-shaped mirror
  logic.
- The next A2DP convergence target remains replacing mirror code with directly
  imported upstream `create_app()` / application cleanup behavior.

## 2026-06-14 A2DP upstream endpoint request lifecycle closeout

A2DP closeout added a gate for upstream-shaped `media.c` endpoint request
cleanup.  The bridge now validates single request cancel, cancel-all cleanup,
and endpoint destroy cleanup for request, transport, sender/path/uuid, and
adapter endpoint ownership.

Verification artifacts:

- `FeatherCore/build/logs/build-bt1-a2dp-upstream-endpoint-request.log`
- `FeatherCore/build/logs/build-bt2-a2dp-upstream-endpoint-request.log`
- `FeatherCore/build/logs/run-a2dp-upstream-endpoint-request.log`
- `FeatherCore/build/bt-hwsim-usecases-a2dp-upstream-endpoint-request/run-results.json`

Validated hwsim case:

- `bluez-a2dp-upstream-convergence-closeout`: PASS for bt1/source and
  bt2/sink.
- Source and sink both emit `bluez-daemon: a2dp upstream-handler-bridge-surface-wrapper ... upstream-endpoint-request=cancel:1,cancel-all:1,destroy:1,total:3 ... final-ok=1`.
- The validator now rejects an A2DP bridge that lacks upstream-shaped endpoint
  request cancel/cancel-all/destroy lifecycle evidence.

Current boundary:

- Endpoint request cleanup is represented by NuttX-side upstream-shaped mirror
  logic.
- The next A2DP convergence target remains replacing mirror cleanup code with
  directly imported upstream `media_endpoint_cancel*()` and
  `media_endpoint_destroy()` behavior.

## 2026-06-14 A2DP upstream endpoint config policy closeout

A2DP closeout added a gate for upstream-shaped `media.c` endpoint
configuration lifecycle.  The bridge now validates SelectConfiguration request
cleanup, SetConfiguration request/transport binding, and ClearConfiguration
transport detachment/final cleanup.

Verification artifacts:

- `FeatherCore/build/logs/build-bt1-a2dp-upstream-endpoint-config.log`
- `FeatherCore/build/logs/build-bt2-a2dp-upstream-endpoint-config.log`
- `FeatherCore/build/logs/run-a2dp-upstream-endpoint-config.log`
- `FeatherCore/build/bt-hwsim-usecases-a2dp-upstream-endpoint-config/run-results.json`

Validated hwsim case:

- `bluez-a2dp-upstream-convergence-closeout`: PASS for bt1/source and
  bt2/sink.
- Source and sink both emit `bluez-daemon: a2dp upstream-handler-bridge-surface-wrapper ... upstream-endpoint-config=select:1,set:1,clear:1,total:3 ... final-ok=1`.
- The validator now rejects an A2DP bridge that lacks upstream-shaped endpoint
  select/set/clear configuration lifecycle evidence.

Current boundary:

- Endpoint configuration lifecycle is represented by NuttX-side upstream-shaped
  mirror logic.
- The next A2DP convergence target remains replacing mirror policy code with
  directly imported upstream `media.c` endpoint request handlers.

## 2026-06-14 A2DP upstream error policy closeout

A2DP closeout added a gate for upstream-shaped error policy.  The bridge now
validates MediaTransport method errors for already-owned acquire,
try-acquire while in use, release without owner, select while in use, and
unselect without owner.  It also validates Media1 registration errors for
duplicate/missing endpoint, player, and application objects.

Verification artifacts:

- `FeatherCore/build/logs/build-bt1-a2dp-upstream-error-policy.log`
- `FeatherCore/build/logs/build-bt2-a2dp-upstream-error-policy.log`
- `FeatherCore/build/logs/run-a2dp-upstream-error-policy.log`
- `FeatherCore/build/bt-hwsim-usecases-a2dp-upstream-error-policy/run-results.json`

Validated hwsim case:

- `bluez-a2dp-upstream-convergence-closeout`: PASS for bt1/source and
  bt2/sink.
- Source and sink both emit `bluez-daemon: a2dp upstream-handler-bridge-surface-wrapper ... upstream-error-policy=transport-methods:1,media-registration:1,total:2 ... final-ok=1`.
- The validator now rejects an A2DP bridge that lacks upstream-shaped
  transport method and media registration error-policy evidence.

Current boundary:

- Error policy is represented by NuttX-side upstream-shaped mirror logic.
- The next A2DP convergence target remains replacing mirror policy code with
  directly imported upstream method handlers and D-Bus error replies.

## 2026-06-14 A2DP upstream transport ops policy closeout

A2DP closeout added a gate for the upstream BlueZ `media_transport_ops`
dispatch policy.  The bridge now validates A2DP source/sink UUID selection,
presence of owner/resume/suspend/cancel/set_state ops, and an A2DP ops
lifecycle path that drives requesting -> active -> suspending -> idle while
clearing resume/cancel ids.

Verification artifacts:

- `FeatherCore/build/logs/build-bt1-a2dp-upstream-transport-ops.log`
- `FeatherCore/build/logs/build-bt2-a2dp-upstream-transport-ops.log`
- `FeatherCore/build/logs/run-a2dp-upstream-transport-ops.log`
- `FeatherCore/build/bt-hwsim-usecases-a2dp-upstream-transport-ops/run-results.json`

Validated hwsim case:

- `bluez-a2dp-upstream-convergence-closeout`: PASS for bt1/source and
  bt2/sink.
- Source and sink both emit `bluez-daemon: a2dp upstream-handler-bridge-surface-wrapper ... upstream-transport-ops=uuid:1,dispatch:1,lifecycle:1,total:3 ... final-ok=1`.
- The validator now rejects an A2DP MediaTransport bridge that lacks upstream
  transport ops UUID, dispatch, and lifecycle evidence.

Current boundary:

- The compat bridge now mirrors upstream `media_transport_ops` dispatch policy.
- The next A2DP convergence target remains replacing mirror policy code with
  directly imported upstream `transport_ops[]` and A2DP ops callbacks.

## 2026-06-14 A2DP upstream transport state policy closeout

A2DP closeout added a gate for the exact upstream BlueZ `transport.c`
state policy.  The bridge now validates `state2str()` behavior for idle,
pending, broadcasting, requesting, active, and suspending states, validates
`state_in_use()` semantics, and verifies the pending -> requesting -> active
-> suspending -> idle transition path.

Verification artifacts:

- `FeatherCore/build/logs/build-bt1-a2dp-upstream-state-policy.log`
- `FeatherCore/build/logs/build-bt2-a2dp-upstream-state-policy.log`
- `FeatherCore/build/logs/run-a2dp-upstream-state-policy.log`
- `FeatherCore/build/bt-hwsim-usecases-a2dp-upstream-state-policy/run-results.json`

Validated hwsim case:

- `bluez-a2dp-upstream-convergence-closeout`: PASS for bt1/source and
  bt2/sink.
- Source and sink both emit `bluez-daemon: a2dp upstream-handler-bridge-surface-wrapper ... upstream-state-policy=state2str:1,in-use:1,transitions:1,total:3 ... final-ok=1`.
- The validator now rejects an A2DP MediaTransport bridge that lacks the
  upstream state string, in-use, and transition policy evidence.

Current boundary:

- The compat bridge now mirrors upstream `transport.c` state policy.
- The next A2DP convergence target remains replacing mirror policy code with
  directly imported upstream `transport_set_state()` and related callbacks.

## 2026-06-14 A2DP upstream-named object graph closeout

A2DP closeout anchored the NuttX Media/Transport bridge ledger to the actual
upstream BlueZ object graph names from `profiles/audio/media.c` and
`profiles/audio/transport.c`.  The bridge now validates relationships matching
`media_adapter`, `media_endpoint`, `endpoint_request`, `media_transport`,
`media_owner`, `media_request`, `a2dp_transport`, and `media_transport_ops`.

Verification artifacts:

- `FeatherCore/build/logs/build-bt1-a2dp-upstream-object-graph.log`
- `FeatherCore/build/logs/build-bt2-a2dp-upstream-object-graph.log`
- `FeatherCore/build/logs/run-a2dp-upstream-object-graph.log`
- `FeatherCore/build/bt-hwsim-usecases-a2dp-upstream-object-graph/run-results.json`

Validated hwsim case:

- `bluez-a2dp-upstream-convergence-closeout`: PASS for bt1/source and
  bt2/sink.
- Source and sink both emit `bluez-daemon: a2dp upstream-handler-bridge-surface-wrapper ... upstream-object-graph=media:1,transport:1,endpoint-request:1,total:3 ... final-ok=1`.
- The validator now rejects a Media/Transport bridge that lacks an
  upstream-named object graph relationship gate.

Current boundary:

- The compat graph now mirrors upstream object names and ownership edges.
- The next A2DP convergence target remains replacing these mirror structs with
  directly imported upstream `media.c` / `transport.c` structures and handlers.

## 2026-06-14 A2DP D-Bus request/error lifecycle semantic closeout

A2DP closeout advanced the BlueZ Media/Transport bridge ownership model with
explicit D-Bus request lifecycle semantics.  The bridge now validates successful
Acquire/Release-style replies, TryAcquire busy/error reply cleanup, duplicate
registration rejection, missing-object rejection, owner-disconnect cleanup, and
no leaked pending request/message refs.

Verification artifacts:

- `FeatherCore/build/logs/build-bt1-a2dp-lifecycle-semantics.log`
- `FeatherCore/build/logs/build-bt2-a2dp-lifecycle-semantics.log`
- `FeatherCore/build/logs/run-a2dp-lifecycle-semantics.log`
- `FeatherCore/build/bt-hwsim-usecases-a2dp-lifecycle-semantics/run-results.json`

Validated hwsim case:

- `bluez-a2dp-upstream-convergence-closeout`: PASS for bt1/source and
  bt2/sink.
- Source and sink both emit `bluez-daemon: a2dp upstream-handler-bridge-surface-wrapper ... lifecycle-semantics=dbus-requests:1,errors:1,total:2 ... final-ok=1`.
- The validator now rejects Media/Transport ownership that lacks explicit
  successful request and error lifecycle evidence.

Current boundary:

- D-Bus request/error lifecycle behavior is represented by NuttX-side
  upstream-shaped semantic ledgers.
- The next A2DP convergence target remains replacing these ledgers with direct
  imported upstream `profiles/audio/media.c` / `profiles/audio/transport.c`
  request objects and callbacks.

## 2026-06-14 A2DP Media/Transport ownership semantic closeout

A2DP closeout advanced the BlueZ Media/Transport bridge from handler semantics
to object and request ownership semantics.  The bridge now validates D-Bus
object registration/release, owner watch cleanup, pending request ownership,
message ref/reply cleanup, fd handoff, and final-zero lifecycle accounting.

Verification artifacts:

- `FeatherCore/build/logs/build-bt1-a2dp-ownership-semantics.log`
- `FeatherCore/build/logs/build-bt2-a2dp-ownership-semantics.log`
- `FeatherCore/build/logs/run-a2dp-ownership-semantics.log`
- `FeatherCore/build/bt-hwsim-usecases-a2dp-ownership-semantics/run-results.json`

Validated hwsim case:

- `bluez-a2dp-upstream-convergence-closeout`: PASS for bt1/source and
  bt2/sink.
- Source and sink both emit `bluez-daemon: a2dp upstream-handler-bridge-surface-wrapper ... ownership-semantics=objects:1,requests:1,final-zero:1,total:3 ... final-ok=1`.
- The validator now rejects a Media/Transport bridge that lacks object,
  request, and final-zero ownership evidence.

Current boundary:

- Media/Transport object and request ownership are represented by NuttX-side
  upstream-shaped semantic ledgers.
- The next A2DP convergence target remains replacing these ledgers with direct
  imported upstream `profiles/audio/media.c` / `profiles/audio/transport.c`
  object and request structures.

## 2026-06-14 A2DP Media1 semantic wrapper closeout

A2DP closeout advanced the BlueZ `Media1` bridge side from callable-only
coverage to explicit lifecycle semantics.  The bridge now validates
`RegisterEndpoint`, `UnregisterEndpoint`, `RegisterPlayer`,
`UnregisterPlayer`, `RegisterApplication`, and `UnregisterApplication`, plus
`SupportedUUIDs` and `SupportedFeatures` property getters.

Verification artifacts:

- `FeatherCore/build/logs/build-bt1-a2dp-media-semantics.log`
- `FeatherCore/build/logs/build-bt2-a2dp-media-semantics.log`
- `FeatherCore/build/logs/run-a2dp-media-semantics.log`
- `FeatherCore/build/bt-hwsim-usecases-a2dp-media-semantics/run-results.json`

Validated hwsim case:

- `bluez-a2dp-upstream-convergence-closeout`: PASS for bt1/source and
  bt2/sink.
- Source and sink both emit `bluez-daemon: a2dp upstream-handler-bridge-surface-wrapper ... media-semantics=methods:6,properties:2,total:8 ... final-ok=1`.
- The validator now rejects a Media1 handler family that only exposes callable
  method/property symbols without registration and property semantics.

Current boundary:

- MediaTransport and Media1 handler families now both have NuttX-side
  upstream-shaped compat semantics.
- The next A2DP convergence target remains direct imported upstream
  `profiles/audio/media.c` / `profiles/audio/transport.c` D-Bus object and
  request ownership.

## 2026-06-14 A2DP transport property semantic wrapper closeout

A2DP closeout advanced the MediaTransport property bridge from callable-only
coverage to explicit D-Bus property semantics.  The bridge now validates
getter semantics for `Device`, `UUID`, `Codec`, `Configuration`, `State`,
`Delay`, `Volume`, and experimental `Endpoint`, setter semantics for `Delay`
and `Volume`, plus existence predicates for `Delay`, `Volume`, and
`Endpoint`.

Verification artifacts:

- `FeatherCore/build/logs/build-bt1-a2dp-transport-property-semantics.log`
- `FeatherCore/build/logs/build-bt2-a2dp-transport-property-semantics.log`
- `FeatherCore/build/logs/run-a2dp-transport-property-semantics.log`
- `FeatherCore/build/bt-hwsim-usecases-a2dp-transport-property-semantics/run-results.json`

Validated hwsim case:

- `bluez-a2dp-upstream-convergence-closeout`: PASS for bt1/source and
  bt2/sink.
- Source and sink both emit `bluez-daemon: a2dp upstream-handler-bridge-surface-wrapper ... property-semantics=getters:8,setters:2,exists:3,total:13 ... final-ok=1`.
- The validator now rejects a MediaTransport property family that only exposes
  callable handlers without property value/set/existence semantics.

Current boundary:

- MediaTransport method and property families now both have NuttX-side
  upstream-shaped compat semantics.
- The next A2DP convergence target remains direct imported upstream
  `profiles/audio/transport.c` object ownership and D-Bus request handling.

## 2026-06-14 A2DP transport full method semantic wrapper closeout

A2DP closeout advanced every MediaTransport method bridge wrapper from
callable-only surface coverage to explicit upstream-shaped lifecycle semantics.
`Acquire`, `TryAcquire`, `Release`, `Select`, and `Unselect` now each execute a
stateful compat path before the bridge can report success.

Verification artifacts:

- `FeatherCore/build/logs/build-bt1-a2dp-transport-full-method-semantics.log`
- `FeatherCore/build/logs/build-bt2-a2dp-transport-full-method-semantics.log`
- `FeatherCore/build/logs/run-a2dp-transport-full-method-semantics.log`
- `FeatherCore/build/bt-hwsim-usecases-a2dp-transport-full-method-semantics/run-results.json`

Validated hwsim case:

- `bluez-a2dp-upstream-convergence-closeout`: PASS for bt1/source and
  bt2/sink.
- Source and sink both emit `bluez-daemon: a2dp upstream-handler-bridge-surface-wrapper ... handler-semantics=acquire:1,try-acquire:1,release:1,select:1,unselect:1,total:5 ... final-ok=1`.
- The validator now rejects a MediaTransport method family that only exposes
  callable symbols without per-method semantics.

Current boundary:

- The complete MediaTransport method family now has NuttX-side compat
  lifecycle semantics.
- The wrappers still need to keep converging toward direct imported upstream
  `profiles/audio/transport.c` handler ownership and D-Bus object state.

## 2026-06-14 A2DP transport acquire/release semantic wrapper closeout

A2DP closeout advanced the MediaTransport method bridge from named-callable
symbols to lifecycle-bearing wrappers for `Acquire` and `Release`.  The bridge
now validates an upstream-shaped transport state transition: idle -> requesting
-> active for acquire, and active -> releasing -> idle for release, including
owner/request/fd/MTU and resume/suspend side effects.

Verification artifacts:

- `FeatherCore/build/logs/build-bt1-a2dp-transport-method-semantics.log`
- `FeatherCore/build/logs/build-bt2-a2dp-transport-method-semantics.log`
- `FeatherCore/build/logs/run-a2dp-transport-method-semantics.log`
- `FeatherCore/build/bt-hwsim-usecases-a2dp-transport-method-semantics/run-results.json`

Validated hwsim case:

- `bluez-a2dp-upstream-convergence-closeout`: PASS for bt1/source and
  bt2/sink.
- Source and sink both emit `bluez-daemon: a2dp upstream-handler-bridge-surface-wrapper ... handler-semantics=acquire:1,release:1,acquire-release:2 ... final-ok=1`.
- The validator now requires this semantic wrapper evidence, so generic
  callable symbols alone no longer satisfy the A2DP bridge gate.

Current boundary:

- `Acquire` and `Release` now carry explicit MediaTransport lifecycle semantics
  in the NuttX bridge.
- The wrappers are still compat implementations, not unmodified upstream
  `profiles/audio/transport.c` handler bodies.

## 2026-06-14 A2DP transport method wrapper symbol closeout

A2DP closeout replaced the first handler-family generic bridge stub with named callable wrapper symbols.  The MediaTransport method bridge entries in `bluez/upstream_media_transport_bridge.c` now point to separate wrapper symbols for `acquire`, `try_acquire`, `release`, `select_transport`, and `unselect_transport`, instead of sharing one generic handler stub.

Verification artifacts:

- `FeatherCore/build/logs/build-bt1-a2dp-transport-method-symbols.log`
- `FeatherCore/build/logs/build-bt2-a2dp-transport-method-symbols.log`
- `FeatherCore/build/logs/run-a2dp-transport-method-symbols.log`
- `FeatherCore/build/bt-hwsim-usecases-a2dp-transport-method-symbols/run-results.json`

Validated hwsim case:

- `bluez-a2dp-upstream-convergence-closeout`: PASS for bt1/source and bt2/sink.
- Source and sink both emit `bluez-daemon: a2dp upstream-handler-bridge-surface-wrapper ... handler-symbols=transport-methods:5 ... final-ok=1`.
- Confirmed all five MediaTransport method wrapper symbols compile, link, and are callable through the bridge ABI.
- Confirmed the existing handler-call coverage remains intact: transport methods, transport properties, media methods, and media properties still all pass.

Current boundary:

- This is the first concrete replacement of a generic bridge stub with named wrapper symbols matching upstream handler families.
- The wrappers still call compat bridge logic, not unmodified upstream `transport.c` handler bodies.
- Next A2DP step should replace one named wrapper body, starting with `acquire` or `release`, with a wrapper around directly imported upstream handler semantics.

## 2026-06-14 A2DP handler symbol surface closeout

A2DP closeout advanced the dedicated Media/Transport bridge module from a presence map to a callable handler-symbol surface.  Each bridge entry in `bluez/upstream_media_transport_bridge.c` now carries a handler function pointer, and the hwsim gate validates that every mapped Media/Transport handler family is actually callable through the bridge ABI.

Verification artifacts:

- `FeatherCore/build/logs/build-bt1-a2dp-handler-symbol-surface.log`
- `FeatherCore/build/logs/build-bt2-a2dp-handler-symbol-surface.log`
- `FeatherCore/build/logs/run-a2dp-handler-symbol-surface.log`
- `FeatherCore/build/bt-hwsim-usecases-a2dp-handler-symbol-surface/run-results.json`

Validated hwsim case:

- `bluez-a2dp-upstream-convergence-closeout`: PASS for bt1/source and bt2/sink.
- Source and sink both emit `bluez-daemon: a2dp upstream-handler-bridge-surface-wrapper ... handler-calls=... symbols-callable:1 ... final-ok=1`.
- Confirmed callable transport method symbols: acquire, try-acquire, release, select, and unselect.
- Confirmed callable transport property symbols: getters, setters, and exists predicates.
- Confirmed callable media method symbols: register/unregister endpoint, register/unregister player, and register/unregister application.
- Confirmed callable media property symbols: supported UUIDs and supported features.

Current boundary:

- This narrows the replacement seam: bridge entries are no longer only named/present, they are callable through a uniform ABI.
- The callable symbols are still compat bridge stubs, not direct unmodified upstream `media.c`/`transport.c` handlers.
- Next A2DP step should replace the bridge stub for one handler family at a time with a wrapper around directly compiled/imported upstream logic.

## 2026-06-14 A2DP handler bridge module closeout

A2DP closeout moved the Media/Transport handler-family bridge out of the large `upstream_a2dp_compat.c` print/runner body into a dedicated replaceable ABI module.  The new `bluez/upstream_media_transport_bridge.c` and `bluez/upstream_media_transport_bridge.h` now own the handler bridge maps and expose `bluez_upstream_a2dp_handler_bridge_surface_run()` for the daemon compatibility path.

Verification artifacts:

- `FeatherCore/build/logs/build-bt1-a2dp-handler-bridge-module.log`
- `FeatherCore/build/logs/build-bt2-a2dp-handler-bridge-module.log`
- `FeatherCore/build/logs/run-a2dp-handler-bridge-module.log`
- `FeatherCore/build/bt-hwsim-usecases-a2dp-handler-bridge-module/run-results.json`

Validated hwsim case:

- `bluez-a2dp-upstream-convergence-closeout`: PASS for bt1/source and bt2/sink.
- Source and sink both still emit `bluez-daemon: a2dp upstream-handler-bridge-surface-wrapper ... final-ok=1`.
- Confirmed the new module is included by both `apps/wireless/linux_bluetooth/Makefile` and `apps/wireless/linux_bluetooth/CMakeLists.txt`.
- Confirmed `upstream_a2dp_compat.c` now consumes the bridge through `upstream_media_transport_bridge.h` instead of owning the handler maps inline.

Current boundary:

- This is a structural step toward replacing staged dispatch: the handler-family bridge is now an isolated ABI seam that can be swapped for real upstream `media.c`/`transport.c` handler implementations.
- It still does not directly link unmodified upstream handler bodies.
- Next A2DP step should start replacing individual bridge entries inside `upstream_media_transport_bridge.c` with wrapper calls around directly compiled/imported upstream handler code.

## 2026-06-14 A2DP handler bridge map refactor closeout

A2DP closeout kept the same upstream handler-family coverage but refactored the Media/Transport handler bridge from hard-coded runner counters into explicit data-driven bridge maps in `bluez/upstream_a2dp_compat.c`.  This is a pre-replacement step: each bridge entry now names the upstream handler family that should later be replaced by a directly compiled/imported `media.c` or `transport.c` handler.

Verification artifacts:

- `FeatherCore/build/logs/build-bt1-a2dp-handler-bridge-map.log`
- `FeatherCore/build/logs/build-bt2-a2dp-handler-bridge-map.log`
- `FeatherCore/build/logs/run-a2dp-handler-bridge-map.log`
- `FeatherCore/build/bt-hwsim-usecases-a2dp-handler-bridge-map/run-results.json`

Validated hwsim case:

- `bluez-a2dp-upstream-convergence-closeout`: PASS for bt1/source and bt2/sink.
- Source and sink both still emit `bluez-daemon: a2dp upstream-handler-bridge-surface-wrapper ... final-ok=1`.
- Confirmed handler bridge maps now drive the transport method, transport property getter/setter/exists, media method, and media property getter counts.
- Confirmed external hwsim contract did not change while the internal compat representation became easier to replace with real upstream handlers.

Current boundary:

- This reduces staged glue shape by collecting handler-family presence into explicit maps, instead of scattering counts through the runner body.
- It still does not directly link unmodified upstream `media.c`/`transport.c` handlers.
- Next A2DP step should start replacing individual bridge-map entries with real handler symbols or wrapper calls around directly compiled upstream handler code.

## 2026-06-14 A2DP upstream handler bridge surface closeout

A2DP closeout advanced from D-Bus table surface mirroring into an explicit upstream handler-family bridge surface for `third/bluez/profiles/audio/media.c` and `third/bluez/profiles/audio/transport.c`.  The NuttX sim BlueZ daemon path now validates that each mirrored MediaTransport/MediaEndpoint table entry has a corresponding handler family mapped before replacing staged dispatch with directly imported upstream handlers.

Verification artifacts:

- `FeatherCore/build/logs/build-bt1-a2dp-handler-bridge-surface.log`
- `FeatherCore/build/logs/build-bt2-a2dp-handler-bridge-surface.log`
- `FeatherCore/build/logs/run-a2dp-handler-bridge-surface.log`
- `FeatherCore/build/bt-hwsim-usecases-a2dp-handler-bridge-surface/run-results.json`

Validated hwsim case:

- `bluez-a2dp-upstream-convergence-closeout`: PASS for bt1/source and bt2/sink.
- Source and sink both emit `bluez-daemon: a2dp upstream-handler-bridge-surface-wrapper ... final-ok=1`.
- Confirmed MediaTransport method handler families: acquire, try-acquire, release, select, and unselect.
- Confirmed MediaTransport property getter families: device, uuid, codec, configuration, state, delay, volume, and endpoint.
- Confirmed MediaTransport property setter/exists families: delay setter, volume setter, delay exists, volume exists, and endpoint exists.
- Confirmed Media method handler families: register/unregister endpoint, register/unregister player, and register/unregister application.
- Confirmed Media property getter families: supported UUIDs and supported features.

Current boundary:

- This is a stronger pre-replacement boundary than the earlier table-surface mirror: every upstream D-Bus table family now has an explicit handler-family mapping in the hwsim gate.
- It is still a handler-family bridge in `bluez/upstream_a2dp_compat.c`, not direct linking of the unmodified upstream `media.c`/`transport.c` handlers.
- Next A2DP step should begin replacing these mapped handler families with directly compiled/imported handler functions and route the staged D-Bus calls through those functions.

## 2026-06-14 A2DP upstream D-Bus table surface closeout

A2DP closeout advanced from MediaEndpoint/MediaTransport dispatch semantics into the upstream D-Bus table surface from `third/bluez/profiles/audio/media.c` and `third/bluez/profiles/audio/transport.c`.  The NuttX sim BlueZ daemon path now validates the method/property table shape needed before replacing staged dispatch with directly imported upstream method handlers.

Verification artifacts:

- `FeatherCore/build/logs/build-bt1-a2dp-dbus-table-surface.log`
- `FeatherCore/build/logs/build-bt2-a2dp-dbus-table-surface.log`
- `FeatherCore/build/logs/run-a2dp-dbus-table-surface.log`
- `FeatherCore/build/bt-hwsim-usecases-a2dp-dbus-table-surface/run-results.json`

Validated hwsim case:

- `bluez-a2dp-upstream-convergence-closeout`: PASS for bt1/source and bt2/sink.
- Source and sink both emit `bluez-daemon: a2dp upstream-dbus-table-surface-wrapper ... final-ok=1`.
- Confirmed `transport_methods`: `Acquire`, `TryAcquire`, `Release`, `Select`, and `Unselect` are all represented as async methods.
- Confirmed `transport_a2dp_properties`: `Device`, `UUID`, `Codec`, `Configuration`, `State`, `Delay`, `Volume`, and experimental `Endpoint`.
- Confirmed writable/existing A2DP properties: `Delay` setter, `Volume` setter, and experimental `Endpoint` property flag.
- Confirmed `media_methods`: `RegisterEndpoint`, `UnregisterEndpoint`, `RegisterPlayer`, `UnregisterPlayer`, `RegisterApplication`, and `UnregisterApplication`.
- Confirmed `media_properties`: `SupportedUUIDs` and `SupportedFeatures`.
- Confirmed transport ops surface: A2DP source/sink plus BAP unicast/broadcast ops are mirrored for profile table alignment.

Current boundary:

- This tightens the A2DP daemon/profile alignment by checking the actual upstream D-Bus table surface, including methods/properties previously not covered by the staged dispatch probes.
- It is still a mirrored table-surface check in `bluez/upstream_a2dp_compat.c`, not the unmodified upstream `media.c`/`transport.c` tables registered directly in the daemon.
- Next A2DP step should replace the mirrored surface with directly compiled/imported method tables and route hwsim D-Bus calls through those upstream handlers.

## 2026-06-14 A2DP upstream MediaEndpoint D-Bus closeout

A2DP closeout advanced from MediaTransport method dispatch into the Media endpoint registration/configuration surface used by upstream `third/bluez/profiles/audio/media.c`.  The NuttX sim BlueZ daemon path now validates a staged dispatch slice for `org.bluez.Media1` and `org.bluez.MediaEndpoint1`, covering endpoint registration, property parsing, SEP creation, async Select/SetConfiguration, ClearConfiguration, Release, unregister, and cleanup.

Verification artifacts:

- `FeatherCore/build/logs/build-bt1-a2dp-media-endpoint-dbus.log`
- `FeatherCore/build/logs/build-bt2-a2dp-media-endpoint-dbus.log`
- `FeatherCore/build/logs/run-a2dp-media-endpoint-dbus.log`
- `FeatherCore/build/bt-hwsim-usecases-a2dp-media-endpoint-dbus/run-results.json`

Validated hwsim case:

- `bluez-a2dp-upstream-convergence-closeout`: PASS for bt1/source and bt2/sink.
- Source and sink both emit `bluez-daemon: a2dp upstream-media-endpoint-dbus-wrapper ... final-ok=1`.
- Confirmed endpoint registration surface: `RegisterEndpoint`, `UnregisterEndpoint`, UUID/codec/capability/delay-reporting parsing, duplicate rejection, invalid UUID rejection, and invalid capability rejection.
- Confirmed SEP binding: source SEP creation, sink SEP creation, and SEP removal.
- Confirmed endpoint method flow: `SelectConfiguration`, `SetConfiguration`, `ClearConfiguration`, `Release`, async request creation, pending call tracking, success reply, error reply, and request cancellation.
- Confirmed transport binding: SetConfiguration creates/appends a transport, ClearConfiguration clears/destroys it.
- Confirmed cleanup: endpoint remove, endpoint destroy, owner watch remove, custom property remove, and pending endpoints/requests/transports/watches all return to zero.

Current boundary:

- This moves A2DP above MediaTransport D-Bus dispatch into the Media1/MediaEndpoint1 profile registration contract used by real BlueZ audio clients.
- It is still a staged callable slice in `bluez/upstream_a2dp_compat.c`, not the unmodified upstream `media.c` method handlers and mainloop dispatch linked end-to-end.
- Next A2DP step should progressively replace this staged endpoint dispatch slice with directly compiled/imported `media.c` handlers and route real D-Bus calls through the daemon path.

## 2026-06-14 A2DP upstream MediaTransport D-Bus dispatch closeout

A2DP closeout advanced from MediaTransport ownership into the daemon D-Bus method surface used by upstream `third/bluez/profiles/audio/transport.c`.  The NuttX sim BlueZ daemon path now validates a staged dispatch slice for `org.bluez.MediaTransport1` covering `Acquire`, `TryAcquire`, `Release`, property get/set, success replies, state guards, error replies, request lifecycle, and owner watch cleanup.

Verification artifacts:

- `FeatherCore/build/logs/build-bt1-a2dp-media-transport-dbus.log`
- `FeatherCore/build/logs/build-bt2-a2dp-media-transport-dbus.log`
- `FeatherCore/build/logs/run-a2dp-media-transport-dbus.log`
- `FeatherCore/build/bt-hwsim-usecases-a2dp-media-transport-dbus/run-results.json`

Validated hwsim case:

- `bluez-a2dp-upstream-convergence-closeout`: PASS for bt1/source and bt2/sink.
- Source and sink both emit `bluez-daemon: a2dp upstream-media-transport-dbus-wrapper ... final-ok=1`.
- Confirmed D-Bus method surface: `Acquire`, `TryAcquire`, and `Release` are represented with success and guarded failure paths.
- Confirmed successful client flow: fd/imtu/omtu reply, request completion, state transitions through idle/requesting/active/suspending/idle, and owner watch add/remove.
- Confirmed error flow: owner conflict, not available, not authorized, invalid args, and unsupported property/method behavior.
- Confirmed property flow: get properties, set volume, set delay, volume changed signal, and delay changed signal.
- Confirmed cleanup: pending owners, requests, fds, and watches all return to zero.

Current boundary:

- This moves A2DP above MediaTransport object ownership into the real D-Bus method contract that BlueZ clients use for audio transport acquisition and release.
- It is still a staged callable slice in `bluez/upstream_a2dp_compat.c`, not the unmodified upstream `transport.c` method table and mainloop dispatch linked end-to-end.
- Next A2DP step should replace this staged D-Bus dispatch slice with directly compiled/imported `transport.c` method handlers and then bind real daemon mainloop ownership to the hwsim AVDTP path.

## 2026-06-14 A2DP upstream MediaTransport ownership closeout

A2DP closeout advanced from AVDTP Close/Abort teardown into BlueZ daemon-side MediaEndpoint/MediaTransport ownership semantics from upstream `third/bluez/profiles/audio/media.c` and `third/bluez/profiles/audio/transport.c`.  The NuttX sim BlueZ daemon path now validates a staged ownership slice covering `org.bluez.MediaEndpoint1`, `org.bluez.MediaTransport1`, transport creation, owner/watch lifecycle, Acquire/Release, A2DP resume/suspend binding, properties, and destroy/unregister cleanup.

Verification artifacts:

- `FeatherCore/build/logs/build-bt1-a2dp-media-transport-ownership.log`
- `FeatherCore/build/logs/build-bt2-a2dp-media-transport-ownership.log`
- `FeatherCore/build/logs/run-a2dp-media-transport-ownership.log`
- `FeatherCore/build/bt-hwsim-usecases-a2dp-media-transport-ownership/run-results.json`

Validated hwsim case:

- `bluez-a2dp-upstream-convergence-closeout`: PASS for bt1/source and bt2/sink.
- Source and sink both emit `bluez-daemon: a2dp upstream-media-transport-ownership-wrapper ... final-ok=1`.
- Confirmed MediaEndpoint/MediaTransport object lifecycle: endpoint registration/find, transport create/path allocation, A2DP ops lookup/init, configuration copy, D-Bus register, global transport append, endpoint transport append, and property export.
- Confirmed owner lifecycle: owner creation, disconnect watch add, owner set/remove, pending request removal, clear owner, and final owner/watch zero.
- Confirmed Acquire path: request, `TRANSPORT_STATE_REQUESTING`, `transport_a2dp_resume`, fd ready, fd/imtu/omtu reply, and `TRANSPORT_STATE_ACTIVE`.
- Confirmed Release/cancel path: Release request, `transport_a2dp_suspend`, `TRANSPORT_STATE_SUSPENDING`, `TRANSPORT_STATE_IDLE`, cancel-resume, and `a2dp_cancel`.
- Confirmed properties and cleanup: delay update/emit, volume get/set/emit, clear configuration, endpoint remove transport, cancel all, transport destroy, D-Bus unregister, transport free, and final pending transports zero.

Current boundary:

- This moves A2DP above AVDTP transaction probes into daemon-side MediaTransport ownership semantics required by real BlueZ clients using `Acquire` and `Release`.
- It is still a staged callable slice in `bluez/upstream_a2dp_compat.c`, not the unmodified upstream `media.c`/`transport.c` object body linked end-to-end.
- Next A2DP step should progressively replace this staged ownership slice with directly compiled/imported `media.c`/`transport.c` code paths and real daemon mainloop/D-Bus method dispatch.

## 2026-06-14 A2DP upstream Close/Abort transaction closeout

A2DP closeout advanced from Start/Suspend lifecycle into the Close/Abort and cancellation paths used by upstream `third/bluez/profiles/audio/a2dp.c`.  The NuttX sim BlueZ daemon path now validates the staged transaction flow around `close_ind`, `close_cfm`, `abort_ind`, `abort_cfm`, `a2dp_cancel`, and `a2dp_reconfigure`.

Verification artifacts:

- `FeatherCore/build/logs/build-bt1-a2dp-close-abort-transaction.log`
- `FeatherCore/build/logs/build-bt2-a2dp-close-abort-transaction.log`
- `FeatherCore/build/logs/run-a2dp-close-abort-transaction.log`
- `FeatherCore/build/bt-hwsim-usecases-a2dp-close-abort-transaction/run-results.json`

Validated hwsim case:

- `bluez-a2dp-upstream-convergence-closeout`: PASS for bt1/source and bt2/sink.
- Source and sink both emit `bluez-daemon: a2dp upstream-close-abort-transaction-wrapper ... final-ok=1`.
- Confirmed Close paths: Close_Ind unwinds pending suspend/resume with `-ECONNRESET`, Close_Cfm success resolves remote SEP and schedules reconfigure, and Close_Cfm error clears stream and finalizes config.
- Confirmed Abort paths: Abort_Ind destroys the stream and unwinds suspend/resume/config callbacks, Abort_Cfm either schedules reconfigure or unreferences setup.
- Confirmed cancellation path: `a2dp_cancel` lookup, setup ref, callback free, AVDTP Abort, and return-after-abort are covered.
- Confirmed cleanup: setup callback free reaches six calls, setup unref reaches four calls, and pending callbacks, setups, streams, and transactions return to zero.

Current boundary:

- This moves A2DP beyond media Start/Suspend into executable Close/Abort/reconfigure/cancel lifecycle semantics required for teardown and error recovery.
- It is still a staged callable slice in `bluez/upstream_a2dp_compat.c`, not the unmodified upstream `a2dp.c` body owning real BlueZ objects end-to-end.
- Next A2DP step should reduce staged ownership around real upstream setup/session/stream structs, then bind these transaction slices to daemon/mainloop/D-Bus MediaTransport ownership.

## 2026-06-14 A2DP upstream Start/Suspend transaction closeout

A2DP closeout advanced from finalizer/callback ABI into the Start/Suspend transaction lifecycle used by upstream `third/bluez/profiles/audio/a2dp.c`.  The NuttX sim BlueZ daemon path now validates the staged transaction flow around `a2dp_resume`, `a2dp_suspend`, `start_ind`, `start_cfm`, `suspend_ind`, and `suspend_cfm`.

Verification artifacts:

- `FeatherCore/build/logs/build-bt1-a2dp-start-suspend-transaction.log`
- `FeatherCore/build/logs/build-bt2-a2dp-start-suspend-transaction.log`
- `FeatherCore/build/logs/run-a2dp-start-suspend-transaction.log`
- `FeatherCore/build/bt-hwsim-usecases-a2dp-start-suspend-transaction/run-results.json`

Validated hwsim case:

- `bluez-a2dp-upstream-convergence-closeout`: PASS for bt1/source and bt2/sink.
- Source and sink both emit `bluez-daemon: a2dp upstream-start-suspend-transaction-wrapper ... final-ok=1`.
- Confirmed resume paths: CONFIGURED defers start until Start_Ind, OPEN issues AVDTP Start and finalizes on Start_Cfm, STREAMING finalizes immediately, and suspend-in-progress defers resume until suspend completion.
- Confirmed suspend paths: OPEN finalizes immediately, STREAMING issues AVDTP Suspend, Suspend_Ind and Suspend_Cfm both finalize suspend callbacks, and restart-after-suspend is covered.
- Confirmed failure paths: resume bad state, suspend bad state, reconfigure reject, AVDTP Start failure, AVDTP Suspend failure, and restart-after-suspend failure.
- Confirmed cleanup: setup callback free and setup unref both reach fifteen calls, with pending callbacks, pending setups, and pending transactions returning to zero.

Current boundary:

- This moves A2DP beyond callback ABI into executable Start/Suspend transaction lifecycle semantics required for media start, suspend, restart, and error unwind.
- It is still a staged callable slice in `bluez/upstream_a2dp_compat.c`, not the unmodified upstream `a2dp.c` body owning real BlueZ objects end-to-end.
- Next A2DP step should reduce staged ownership around actual upstream setup/session/stream structures and then attach the daemon/mainloop/D-Bus ownership path more directly to these transaction slices.

## 2026-06-14 A2DP upstream finalizer/callback ABI closeout

A2DP closeout advanced from SetConfiguration transaction lifecycle into the callback/finalizer ABI used by upstream `third/bluez/profiles/audio/a2dp.c`.  The NuttX sim BlueZ daemon path now validates callable slices using the real upstream callback typedefs `a2dp_config_cb_t` and `a2dp_stream_cb_t`, including success delivery, error delivery, finalizer routing, errno propagation, and cleanup accounting.

Verification artifacts:

- `FeatherCore/build/logs/build-bt1-a2dp-finalizer-callback-v3.log`
- `FeatherCore/build/logs/build-bt2-a2dp-finalizer-callback-v3.log`
- `FeatherCore/build/logs/run-a2dp-finalizer-callback-v3.log`
- `FeatherCore/build/bt-hwsim-usecases-a2dp-finalizer-callback-v3/run-results.json`

Validated hwsim case:

- `bluez-a2dp-upstream-convergence-closeout`: PASS for bt1/source and bt2/sink.
- Source and sink both emit `bluez-daemon: a2dp upstream-finalizer-callback-wrapper ... final-ok=1`.
- Confirmed callback dispatch through `a2dp_config_cb_t` and `a2dp_stream_cb_t` for config, resume, and suspend paths.
- Confirmed finalizer routing: `finalize_config`, `finalize_resume`, `finalize_suspend`, and `finalize_setup_errno`.
- Confirmed success/error delivery: config success/error, resume success/error, suspend success/error, plus stream pointer delivery.
- Confirmed error propagation: `-EIO` and `-EINVAL` paths are counted and delivered to the expected callback families.
- Confirmed cleanup: setup callback free and setup unref both reach six calls, with pending callbacks and pending setups returning to zero.

Current boundary:

- This moves A2DP beyond transaction counters into executable callback/finalizer ABI semantics needed by the upstream audio plugin lifecycle.
- It is still a staged callable slice in `bluez/upstream_a2dp_compat.c`, not the unmodified upstream `a2dp.c` body owning real BlueZ objects end-to-end.
- Next A2DP step should connect these callable slices more directly to imported upstream setup/session/stream object ownership, reducing the remaining staged glue around finalizer dispatch.

## 2026-06-14 A2DP upstream SetConfiguration transaction closeout

A2DP closeout advanced from state-policy decisions into SetConfiguration transaction lifecycle semantics.  The NuttX sim BlueZ daemon path now validates the staged transaction flow corresponding to `third/bluez/profiles/audio/a2dp.c` `a2dp_config`, `set_configuration`, configuration confirmation, and `finalize_config` cleanup.

Verification artifacts:

- `FeatherCore/build/logs/build-bt1-a2dp-setconf-transaction.log`
- `FeatherCore/build/logs/build-bt2-a2dp-setconf-transaction.log`
- `FeatherCore/build/logs/run-a2dp-setconf-transaction.log`
- `FeatherCore/build/bt-hwsim-usecases-a2dp-setconf-transaction/run-results.json`

Validated hwsim case:

- `bluez-a2dp-upstream-convergence-closeout`: PASS for bt1/source and bt2/sink.
- Source and sink both emit `bluez-daemon: a2dp upstream-setconf-transaction-wrapper ... final-ok=1`.
- Confirmed normal transaction lifecycle: setup get, setup callback add, caps copy, remote SEP resolution, AVDTP SetConfiguration, stream assignment, SetConfiguration confirmation, config callback, and finalize.
- Confirmed same-caps path: idle finalize without a new SetConfiguration transaction.
- Confirmed reconfigure path: different caps close the existing stream, set reconfigure flag, retry, then complete SetConfiguration.
- Confirmed failure paths: no remote SEP and AVDTP SetConfiguration failure both cleanup setup callbacks and setup refs.
- Confirmed final cleanup: pending callbacks, pending setups, and pending transactions all zero.

Current boundary:

- This moves A2DP beyond state-policy tables into executable transaction lifecycle semantics required by SetConfiguration and reconfigure paths.
- It is still a staged callable slice in `bluez/upstream_a2dp_compat.c`, not the unmodified upstream `a2dp.c` body owning real BlueZ objects end-to-end.
- Next A2DP step should connect these transaction slices to imported upstream setup/session/stream object code, reducing staged counters around setup callbacks and finalizers.

## 2026-06-14 A2DP upstream SetConfiguration/state policy closeout

A2DP closeout advanced from SEP matching into upstream-style SetConfiguration/session state policy.  The NuttX sim BlueZ daemon path now validates the key state decisions used by `third/bluez/profiles/audio/a2dp.c` around `a2dp_config`, `a2dp_resume`, and `a2dp_suspend`.

Verification artifacts:

- `FeatherCore/build/logs/build-bt1-a2dp-state-policy.log`
- `FeatherCore/build/logs/build-bt2-a2dp-state-policy.log`
- `FeatherCore/build/logs/run-a2dp-state-policy.log`
- `FeatherCore/build/bt-hwsim-usecases-a2dp-state-policy/run-results.json`

Validated hwsim case:

- `bluez-a2dp-upstream-convergence-closeout`: PASS for bt1/source and bt2/sink.
- Source and sink both emit `bluez-daemon: a2dp upstream-state-policy-wrapper ... final-ok=1`.
- Confirmed `a2dp_config` policy: IDLE allows SetConfiguration, OPEN/STREAMING with same caps finalize, OPEN/STREAMING with different caps reconfigure, CONFIGURED/CLOSING/ABORTING reject, locked stream rejects, missing codec rejects, and codec mismatch rejects.
- Confirmed `a2dp_resume` policy: IDLE rejects, CONFIGURED defers start, OPEN starts, STREAMING finalizes, CLOSING/ABORTING reject, and active reconfigure rejects.
- Confirmed `a2dp_suspend` policy: IDLE rejects, OPEN finalizes, STREAMING suspends, CONFIGURED/CLOSING/ABORTING reject, and active reconfigure rejects.

Current boundary:

- This moves A2DP beyond codec/SEP matching into executable session state policy required for SetConfiguration, Start, Suspend, and reconfigure decisions.
- It is still a staged callable slice in `bluez/upstream_a2dp_compat.c`, not the unmodified upstream `a2dp.c` body owning real BlueZ objects end-to-end.
- Next A2DP step should connect these policy slices to more imported upstream object code, reducing staged wrappers around setup/session/stream state transitions.

## 2026-06-14 A2DP upstream SEP matching closeout

A2DP closeout advanced from SBC codec/config selection into upstream-style SEP matching semantics.  The NuttX sim BlueZ daemon path now validates the key matching rules used around `third/bluez/profiles/audio/a2dp.c` `find_sep`, `find_remote_sep`, and `SetConfiguration`: remote SEP direction maps to the opposite local SEP role, MediaEndpoint sender/path must match, codec must match, and invalid combinations are rejected.

Verification artifacts:

- `FeatherCore/build/logs/build-bt1-a2dp-sep-matching.log`
- `FeatherCore/build/logs/build-bt2-a2dp-sep-matching.log`
- `FeatherCore/build/logs/run-a2dp-sep-matching.log`
- `FeatherCore/build/bt-hwsim-usecases-a2dp-sep-matching/run-results.json`

Validated hwsim case:

- `bluez-a2dp-upstream-convergence-closeout`: PASS for bt1/source and bt2/sink.
- Source and sink both emit `bluez-daemon: a2dp upstream-sep-matching-wrapper ... final-ok=1`.
- Confirmed role matching: `remote-source->local-sink=1`, `remote-sink->local-source=1`.
- Confirmed endpoint matching: `sender-path-match=1`.
- Confirmed codec matching: `codec-match=1` for SBC.
- Confirmed rejection paths: wrong sender, wrong path, codec mismatch, missing remote codec, and missing local SEP are all rejected.

Current boundary:

- This moves A2DP beyond codec payload/config selection into executable endpoint/SEP compatibility semantics required before AVDTP SetConfiguration.
- It is still a staged callable slice in `bluez/upstream_a2dp_compat.c`, not the unmodified upstream `a2dp.c` object body owning the full DBus/AVDTP runtime.
- Next A2DP step should continue with SetConfiguration/session state policy, then progressively replace staged helpers with imported upstream BlueZ objects.

## 2026-06-14 A2DP upstream SBC config selector closeout

A2DP closeout advanced from public error mapping into codec/config negotiation semantics.  The NuttX sim BlueZ daemon path now compiles and validates an upstream-style SBC capability selector using `third/bluez/profiles/audio/a2dp-codecs.h` definitions and A2DP protocol error codes.

Verification artifacts:

- `FeatherCore/build/logs/build-bt1-a2dp-sbc-config-selector-v2.log`
- `FeatherCore/build/logs/build-bt2-a2dp-sbc-config-selector-v2.log`
- `FeatherCore/build/logs/run-a2dp-sbc-config-selector-v2.log`
- `FeatherCore/build/bt-hwsim-usecases-a2dp-sbc-config-selector-v2/run-results.json`

Validated hwsim case:

- `bluez-a2dp-upstream-convergence-closeout`: PASS for bt1/source and bt2/sink.
- Source and sink both emit `bluez-daemon: a2dp upstream-sbc-config-selector ... final-ok=1`.
- Confirmed selected SBC config from remote capabilities: `frequency=48000`, `channel=joint-stereo`, `block=16`, `subbands=8`, `alloc=loudness`, `min-bitpool=2`, `max-bitpool=51`.
- Confirmed remote capability input: `frequency=48000+44100`, `channel=joint-stereo+stereo`, `block=16+12`, `subbands=8+4`, `alloc=loudness+snr`, `min-bitpool=2`, `max-bitpool=250`.
- Confirmed error policy: no frequency -> `0xc4`, no channel -> `0xc6`, no block -> `0xdd`, no subbands -> `0xc8`, no allocation -> `0xca`, bad min bitpool -> `0xcb`, bad max bitpool -> `0xcd`, null input -> `0x29`.

Current boundary:

- This moves A2DP beyond media payload probing into executable codec negotiation semantics required by AVDTP SetConfiguration.
- It is still a staged callable slice in `bluez/upstream_a2dp_compat.c`, not the unmodified upstream `a2dp.c` body owning the full endpoint selection path.
- Next A2DP step should continue replacing staged selection and SEP matching helpers with callable upstream slices until the daemon path can rely on imported BlueZ objects rather than compatibility probes.

## 2026-06-14 A2DP upstream config-error parser closeout

A2DP closeout advanced from staged ownership evidence into a callable upstream public-function slice from `third/bluez/profiles/audio/a2dp.c`: `a2dp_parse_config_error()`.  The NuttX sim BlueZ daemon path now compiles and validates the BlueZ A2DP D-Bus error-name to protocol error-code mapping used by MediaEndpoint configuration failure handling.

Verification artifacts:

- `FeatherCore/build/logs/build-bt1-a2dp-config-error-parser.log`
- `FeatherCore/build/logs/build-bt2-a2dp-config-error-parser.log`
- `FeatherCore/build/logs/run-a2dp-config-error-parser.log`
- `FeatherCore/build/bt-hwsim-usecases-a2dp-config-error-parser/run-results.json`

Validated hwsim case:

- `bluez-a2dp-upstream-convergence-closeout`: PASS for bt1/source and bt2/sink.
- Source and sink both emit `bluez-daemon: a2dp upstream-config-error-parser ... final-ok=1`.
- Confirmed mapped BlueZ errors: `InvalidCodecType=0xc1`, `NotSupportedCodecType=0xc2`, `InvalidSamplingFrequency=0xc3`, `InvalidChannelMode=0xc5`, `InvalidMaximumBitpoolValue=0xcd`, `InvalidCodecParameter=0xe2`.
- Confirmed fallback behavior: wrong prefix, unknown suffix, and null input all map to `AVDTP_UNSUPPORTED_CONFIGURATION=0x29`.
- Confirmed upstream table size in the ported slice: `entries=35`.

Current boundary:

- This is a real public A2DP function slice ported into the NuttX BlueZ compat build and required by hwsim validation.
- It is still not the unmodified upstream `a2dp.c` object body owning the whole daemon path.
- Next A2DP step should continue replacing staged helpers with callable upstream slices around config selection, SEP matching, and stream state transition policy.

## 2026-06-14 A2DP upstream AVDTP session/stream ownership closeout

A2DP closeout advanced again from upstream `a2dp.c` setup ownership into a staged ownership model for `third/bluez/profiles/audio/avdtp.c`.  The NuttX sim BlueZ daemon path now emits and validates source/sink lifecycle evidence for AVDTP session, local SEP, remote SEP, discovery callback, pending request, stream, stream callback, transport attach/get/clear, pending-open ownership, state transitions, and final cleanup.

Verification artifacts:

- `FeatherCore/build/logs/build-bt1-a2dp-avdtp-owner.log`
- `FeatherCore/build/logs/build-bt2-a2dp-avdtp-owner.log`
- `FeatherCore/build/logs/run-a2dp-avdtp-owner.log`
- `FeatherCore/build/bt-hwsim-usecases-a2dp-avdtp-owner/run-results.json`

Validated hwsim case:

- `bluez-a2dp-upstream-convergence-closeout`: PASS for bt1/source and bt2/sink.
- Source and sink both emit `bluez-daemon: a2dp upstream-avdtp-ownership-wrapper ... final-ok=1`.
- Confirmed object lifecycle: `session:1`, `local-sep:1`, `remote-sep:1`, `discover:1`, `request:1`, `stream:1`, `stream-cb:1`.
- Confirmed state lifecycle: `configured:1`, `open:1`, `streaming:1`, `idle:1`.
- Confirmed transport lifecycle: `set:1`, `get:1`, `clear:1`, `pending-open-set:1`, `pending-open-clear:1`.
- Confirmed cleanup and leak closure: `discover_free:1`, `remote-sep-unregister:1`, `stream-cb-remove:1`, `stream_free:1`, `session_free:1`, `final-zero=sessions:0,local-seps:0,remote-seps:0,streams:0,discovers:0,requests:0,stream-cbs:0,transports:0,refs:0`.

Current boundary:

- A2DP staged convergence now validates upstream source manifest, public A2DP/AVDTP headers, endpoint callbacks, AVDTP cfm/ind callbacks, `a2dp.c` ownership model, and `avdtp.c` ownership model.
- It is still not an unmodified upstream `bluetoothd` audio plugin with native `a2dp.c` and `avdtp.c` bodies owning the whole runtime.
- Next A2DP step should replace staged ownership probes with progressively larger callable slices from upstream `a2dp.c`/`avdtp.c`, while preserving these hwsim source/sink contracts.

## 2026-06-14 A2DP upstream setup/SEP/stream ownership closeout

A2DP closeout advanced from callback-surface compatibility into a staged ownership model for the upstream `third/bluez/profiles/audio/a2dp.c` object lifecycle.  The NuttX sim BlueZ daemon path now emits and validates a source/sink ownership wrapper for the same core object families used by upstream BlueZ: server, channel, setup, setup callback, SEP, stream, endpoint queues, transport attach/detach, callback completion, and final cleanup.

Verification artifacts:

- `FeatherCore/build/logs/build-bt1-a2dp-setup-owner.log`
- `FeatherCore/build/logs/build-bt2-a2dp-setup-owner.log`
- `FeatherCore/build/logs/run-a2dp-setup-owner.log`
- `FeatherCore/build/bt-hwsim-usecases-a2dp-setup-owner/run-results.json`

Validated hwsim case:

- `bluez-a2dp-upstream-convergence-closeout`: PASS for bt1/source and bt2/sink.
- Source and sink both emit `bluez-daemon: a2dp upstream-setup-ownership-wrapper ... final-ok=1`.
- Confirmed object lifecycle: `server:1`, `channel:1`, `setup:1`, `setup_cb:4`, `sep:1`, `stream:1`.
- Confirmed callback lifecycle: `discover:1`, `select:1`, `config:1`, `resume:1`, `suspend:1`.
- Confirmed transport lifecycle: `attach:1`, `detach:1`.
- Confirmed cleanup and leak closure: `setup_free:1`, `setup_cb_free:4`, `sep_remove:1`, `stream_destroy:1`, `final-zero=setups:0,seps:0,streams:0,cbs:0,refs:0`.

Current boundary:

- This is a staged `a2dp.c` ownership model compiled and driven through the sim daemon, with validator-enforced source/sink evidence.
- It is still not the unmodified upstream `a2dp.c` object body owning the real BlueZ daemon path.
- Next A2DP step should replace the staged ownership probe with progressively larger callable slices from upstream `a2dp.c`, then do the same for `avdtp.c` session/object ownership.

## 2026-06-14 A2DP upstream AVDTP callback wrapper closeout

A2DP closeout advanced one layer beyond the upstream source manifest and `struct a2dp_endpoint` callback wrapper: the NuttX sim BlueZ daemon path now compiles, instantiates, and executes the upstream public `struct avdtp_sep_cfm` and `struct avdtp_sep_ind` callback tables from `bluez/upstream/profiles/audio/avdtp.h` through `bluez/upstream_a2dp_compat.c`.

Verification artifacts:

- `FeatherCore/build/logs/build-bt1-a2dp-avdtp-callback.log`
- `FeatherCore/build/logs/build-bt2-a2dp-avdtp-callback.log`
- `FeatherCore/build/logs/run-a2dp-avdtp-callback.log`
- `FeatherCore/build/bt-hwsim-usecases-a2dp-avdtp-callback/run-results.json`

Validated hwsim case:

- `bluez-a2dp-upstream-convergence-closeout`: PASS for bt1/source and bt2/sink.
- Source and sink both emit `bluez-daemon: a2dp upstream-avdtp-callback-wrapper ... final-ok=1`.
- Confirmed cfm callbacks: `set_configuration`, `get_configuration`, `open`, `start`, `suspend`, `close`, `abort`, `reconfigure`, `delay_report`.
- Confirmed ind callbacks: `match_codec`, `get_capability`, `set_configuration`, `set_configuration_cb`, `get_configuration`, `open`, `start`, `suspend`, `close`, `abort`, `reconfigure`, `delayreport`.
- Existing A2DP closeout still completes full staged media transport, RTP/SBC payload simulation, AVRCP event path, and ownership cleanup.

Current boundary:

- This closes the AVDTP public callback surface alignment for the staged A2DP daemon path.
- It is still not an unmodified upstream `bluetoothd` audio plugin running its native `a2dp.c`/`avdtp.c` object ownership and mainloop end-to-end.
- Next A2DP step should move actual upstream `a2dp.c` setup/session/SEP ownership into the compatibility layer first, then converge `avdtp.c` session/object ownership.

## 2026-06-14 A2DP upstream endpoint callback wrapper closeout

继续 A2DP 去 staged adapter 化，本轮在 upstream header compat wrapper 之后，把 upstream `struct a2dp_endpoint` callback surface 实例化并纳入 hwsim gate。

A2DP upstream convergence + endpoint callback wrapper：PASS

- case: `bluez-a2dp-upstream-convergence-closeout`
- build: `FeatherCore/build/logs/build-bt1-a2dp-endpoint-callback.log`
- build: `FeatherCore/build/logs/build-bt2-a2dp-endpoint-callback.log`
- run: `FeatherCore/build/logs/run-a2dp-endpoint-callback.log`
- manifest: `FeatherCore/build/bt-hwsim-usecases-a2dp-endpoint-callback/run-results.json`
- roles: `bt1` source, `bt2` sink

实现改动：`FeatherCore/apps/wireless/linux_bluetooth/bluez/upstream_a2dp_compat.c` 现在构造真实 upstream public `struct a2dp_endpoint` callback table，并执行 `get_name`、`get_path`、`get_capabilities`、`select_configuration`、`set_configuration`、`clear_configuration`、`set_delay`。`select_configuration` 和 `set_configuration` 分别通过 upstream 类型 `a2dp_endpoint_select_t`、`a2dp_endpoint_config_t` 回调闭环。

新增验证门槛：validator 现在要求 source/sink 两端输出 `bluez-daemon: a2dp upstream-endpoint-callback-wrapper`，并检查 `callbacks=get_name:1,get_path:1,get_capabilities:1,select_configuration:1,select_cb:1,set_configuration:1,set_cb:1,clear_configuration:1,set_delay:1`、`capability-bytes=12`、`selected-bytes=12`、`delay=120`、`final-ok=1`。

当前含义：A2DP upstream-facing 层已从“public headers 可编译”推进到 “public endpoint callback table 可执行”。边界仍明确为 `boundary=upstream-a2dp-endpoint-callbacks-compiled-not-yet-a2dp-c-object` 和 `staged-boundary=bluezdaemon-adapter-not-unmodified-bluetoothd`；下一步继续把 upstream `a2dp.c` 中的 setup/session/SEP owner 逐步拆入 compat wrapper，最终替换当前 `bluezdaemon` staged A2DP object/session owner。

## 2026-06-14 A2DP upstream header compat wrapper closeout

继续 A2DP 去 staged adapter 化，本轮在 upstream source manifest 之后新增可编译的 upstream-facing A2DP compat wrapper。

A2DP upstream convergence + header compat wrapper：PASS

- case: `bluez-a2dp-upstream-convergence-closeout`
- build: `FeatherCore/build/logs/build-bt1-a2dp-upstream-compat-v2.log`
- build: `FeatherCore/build/logs/build-bt2-a2dp-upstream-compat-v2.log`
- run: `FeatherCore/build/logs/run-a2dp-upstream-compat-v2.log`
- manifest: `FeatherCore/build/bt-hwsim-usecases-a2dp-upstream-compat-v2/run-results.json`
- roles: `bt1` source, `bt2` sink

实现改动：新增 `FeatherCore/apps/wireless/linux_bluetooth/bluez/upstream_a2dp_compat.c` 和 `upstream_a2dp_compat.h`，并接入 `CONFIG_LINUX_BLUEZ_DAEMON` 的 Makefile/CMake 构建。该编译单元直接 include `bluez/upstream/profiles/audio/a2dp.h` 和 `bluez/upstream/profiles/audio/avdtp.h`，为 NuttX apps 侧补齐最小 GLib/BlueZ 前置类型 `gboolean`、`GSList`、`GIOChannel`、`GDestroyNotify`、`btd_adapter`、`btd_device`、`queue`。

新增验证门槛：validator 现在要求 source/sink 两端输出 `bluez-daemon: a2dp upstream-compat-wrapper`，并检查 upstream header 里的 callback surface、状态常量、SEP 类型、capability 类型和 error code：`a2dp_endpoint_select_t`、`a2dp_endpoint_config_t`、`a2dp_discover_cb_t`、`a2dp_select_cb_t`、`a2dp_config_cb_t`、`a2dp_stream_cb_t`、`avdtp_session_state_cb`、`avdtp_stream_state_cb`、`avdtp_set_configuration_cb`，以及 `AVDTP_STATE_IDLE/CONFIGURED/OPEN/STREAMING/CLOSING/ABORTING`、`AVDTP_SEP_TYPE_SOURCE/SINK`、`AVDTP_MEDIA_TRANSPORT`、`AVDTP_MEDIA_CODEC`、`AVDTP_DELAY_REPORTING`、`AVDTP_BAD_STATE`、`A2DP_INVALID_CODEC_TYPE`、`A2DP_NOT_SUPPORTED_CODEC_TYPE`、`A2DP_INVALID_CODEC_PARAMETER`。

当前含义：A2DP gate 已从 source mirror/manifest 继续推进到直接编译 upstream A2DP/AVDTP public headers，证明 NuttX apps 侧已有可编译的 upstream-facing callback/type/constant 兼容层。边界仍明确为 `boundary=upstream-headers-compiled-not-yet-upstream-c-object` 和 `staged-boundary=bluezdaemon-adapter-not-unmodified-bluetoothd`；下一步应继续把 `a2dp.c`/`avdtp.c` 的实际 `.c` 对象逐步拆出可编译 compat wrapper，并让真实 callback/session owner 接管更多路径。

## 2026-06-14 A2DP upstream source manifest closeout

继续 A2DP 去 staged adapter 化，本轮确认 `FeatherCore/apps/wireless/linux_bluetooth/bluez/upstream` 是指向 `third/bluez` 的符号链接，并把 A2DP upstream source manifest 编译进 `bluezdaemon`。

A2DP upstream convergence + source manifest：PASS

- case: `bluez-a2dp-upstream-convergence-closeout`
- build: `FeatherCore/build/logs/build-bt1-a2dp-upstream-manifest.log`
- build: `FeatherCore/build/logs/build-bt2-a2dp-upstream-manifest.log`
- run: `FeatherCore/build/logs/run-a2dp-upstream-manifest.log`
- manifest: `FeatherCore/build/bt-hwsim-usecases-a2dp-upstream-manifest/run-results.json`
- roles: `bt1` source, `bt2` sink

实现改动：新增 `FeatherCore/apps/wireless/linux_bluetooth/bluez/upstream_manifest.c` 和 `upstream_manifest.h`，并接入 `CONFIG_LINUX_BLUEZ_DAEMON` 的 Makefile/CMake 构建。`bluezdaemon audio-a2dp-closeout-full` 现在会输出 `bluez-daemon: a2dp upstream-source-manifest`，validator 强制检查 source/sink 两端都看到 `apps-link=bluez/upstream target=third/bluez audio-files=20 core-files=9 compile-unit=bluez/upstream_manifest.c`。

manifest 覆盖的 upstream A2DP/audio 文件包括 `a2dp.c`、`a2dp.h`、`a2dp-codecs.h`、`avdtp.c`、`avdtp.h`、`avctp.c`、`avctp.h`、`avrcp.c`、`avrcp.h`、`avrcp-player.c`、`media.c`、`media.h`、`transport.c`、`transport.h`、`source.c`、`source.h`、`sink.c`、`sink.h`、`player.c`、`player.h`。core 文件包括 `src/main.c`、`src/plugin.c`、`src/profile.c`、`src/device.c`、`src/adapter.c`、`src/dbus-common.c`、`src/sdpd-service.c`、`src/shared/mainloop.c`、`src/shared/io-mainloop.c`。

当前含义：A2DP gate 不再只依赖 `daemon_main.c` 中的 source 字符串，而是要求 `bluezdaemon` 的编译单元显式绑定 apps 侧 upstream BlueZ source mirror。边界仍明确为 `boundary=source-mirror-not-yet-unmodified-plugin` 和 `staged-boundary=bluezdaemon-adapter-not-unmodified-bluetoothd`，因此还不能宣称 unmodified upstream `bluetoothd` audio plugin 已直接运行。下一步继续把 manifest 中的 upstream audio plugin 入口逐步从 source mirror/ledger 推进到可编译 compat wrapper 和真实 callback/session owner。

## 2026-06-14 A2DP upstream daemon ownership ledger closeout

在 BT/BLE NET iperf matrix 收口之后，本轮回到 A2DP，继续把 `bluez-a2dp-upstream-convergence-closeout` 从“子路径覆盖”加严到 “daemon ownership 总账”覆盖。

A2DP upstream convergence + ownership ledger：PASS

- case: `bluez-a2dp-upstream-convergence-closeout`
- build: `FeatherCore/build/logs/build-bt1-a2dp-ownership-ledger.log`
- build: `FeatherCore/build/logs/build-bt2-a2dp-ownership-ledger.log`
- run: `FeatherCore/build/logs/run-a2dp-ownership-ledger.log`
- manifest: `FeatherCore/build/bt-hwsim-usecases-a2dp-ownership-ledger/run-results.json`
- roles: `bt1` source, `bt2` sink

新增验证门槛：`bluezdaemon audio-a2dp-closeout-full` 现在输出并由 validator 强制检查 `bluez-daemon: a2dp closeout upstream-daemon-ownership-ledger`。该 ledger 覆盖 `profile,device,session,stream,media-transport,avrcp-player,l2cap-fd,dbus-name,mainloop-watch` 的 bluetoothd direct owner，并要求两端同时满足：`profile-register=1/profile-unregister=1`、`device-connect=2/device-disconnect=2`、`dbus-name-acquire=1/dbus-name-release=1`、`dbus-owner-lost=1/dbus-owner-reacquire=1`、`mainloop-watch-add=7/mainloop-watch-remove=7`、`mainloop-timer-add=2/mainloop-timer-remove=2`、`avdtp-transactions=12/avdtp-complete=12`、`transport-acquire=2/transport-release=2`、`fd-open=2/fd-close=2`、`zero-ref-rounds=2/rounds=2`，最终 `final-profile-registered=0`、`final-device-ref=0`、`final-session-ref=0`、`final-stream-ref=0`、`final-sep-ref=0`、`final-endpoint-refs=0`、`final-transport-refs=0`、`final-player-refs=0`、`final-dbus-owners=0`、`final-interfaces=0`、`final-mainloop-watches=0`、`final-mainloop-timers=0`、`final-l2cap-fds=0`、`final-media-fd=closed`、`final-transaction-pending=0`、`final-state-errors=0`、`final-ok=1`。

实现改动：`FeatherCore/apps/wireless/linux_bluetooth/bluez/tools/daemon_main.c` 增加 A2DP upstream daemon ownership ledger 计算和输出；`FeatherCore/tools/firmware/sim/validate-bt-hwsim-usecases.py` 对 `bluez-a2dp-upstream-convergence-closeout` 加严，要求 source/sink 两端都出现 ledger 并满足最终归零条件。

边界仍保留：ledger 仍标记 `staged-boundary=bluezdaemon-adapter-not-unmodified-bluetoothd`。因此当前结论是：A2DP 的 daemon/mainloop/D-Bus/object/fd/ref 生命周期审计更强了，但还不能宣称 unmodified upstream `bluetoothd` audio plugin 已完全无 adapter 运行。下一步继续 A2DP 去 staged adapter 化，优先让 upstream `bluetoothd` audio plugin 入口直接拥有 mainloop、D-Bus object、MediaTransport fd 和 AVDTP/L2CAP session。

## 2026-06-14 BT/BLE NET iperf matrix closeout

在 LE Audio full-role、BT/BLE basic、A2DP upstream convergence、BT/BLE NET current closeout 之后，本轮继续补跑并收口 BT Network/BNEP 的 iperf matrix。

BT Network/BNEP iperf matrix：PASS

- case: `bluez-network-iperf-matrix`
- build: `FeatherCore/build/logs/build-bt1-bluez-network-iperf-matrix-final-v3.log`
- build: `FeatherCore/build/logs/build-bt2-bluez-network-iperf-matrix-final-v3.log`
- run: `FeatherCore/build/logs/run-bluez-network-iperf-matrix-final-v3.log`
- manifest: `FeatherCore/build/bt-hwsim-usecases-bluez-network-iperf-matrix-final-v3/run-results.json`
- roles: `bt1`, `bt2`

验证结果：`run-results.json` 显示 `passed=true`、`validate_rc=0`，runner 输出 `PASS bluez-network-iperf-matrix`。BT1/BT2 均完成 BlueZ Network Profile-shaped BNEP/PAN 多轮 TCP/UDP 正反向 iperf，日志中出现非零吞吐，例如约 `0.35` 到 `0.69 Mbits/sec`，并在 native BNEP path 中看到 `bnep-native-sock-ioctl-connadd=4`、`bnep-native-kthread-run=4`、`bnep-native-session-rx-dequeue`、`bnep-native-ndo-start-xmit`、`bnep-native-l2cap-delivered`、`bnep-native-netif-rx` 等非零计数。最终 cleanup 看到 `bnep-native-active=0`、`bnep-native-session-stop=4`、`bnep-native-session-terminate=4`、`bnep-native-netdev-unregister=4`。

实现补充：`linux_bt_upstream_af_status()` 将 `bnep-native-sock-ioctl-connadd/conndel` 提前输出到 BNEP socket ioctl 摘要段，避免长状态行后部被截断导致验证器看不到已经发生的 native BNEP ioctl 证据。该改动不降低验证门槛，只修正证据可见性。

当前阶段结论：LE Audio 当前全角色 gate、BT/BLE basic gate、A2DP upstream convergence gate、BT/BLE NET current closeout、BLE IP ping、BT Network/BNEP iperf matrix 均已通过。下一阶段按用户指定顺序继续补 A2DP 的更深 upstream daemon/mainloop/D-Bus ownership，再补 BT/BLE NET 的 unmodified upstream plugin 收敛，最后补剩余全部 BT/BLE profiles/security/policy。仍保留 staged adapter 边界，不声明整套 upstream Linux/BlueZ 蓝牙栈已经完全无 adapter 移植完。

## 2026-06-14 BT/BLE NET current closeout: BLE IP ping + BlueZ Network/IPSP

本轮进入 BT/BLE NET 阶段，优先验证目标清单里原先标记未完成的 BLE1 <-> BLE2 IP ping，然后运行当前 BT/BLE NET 四角色 closeout。

BLE IP ping：PASS

- case: `ble-ip-ping`
- build: `FeatherCore/build/logs/build-ble1-ble-ip-ping-final.log`
- build: `FeatherCore/build/logs/build-ble2-ble-ip-ping-final.log`
- run: `FeatherCore/build/logs/run-ble-ip-ping-final.log`
- manifest: `FeatherCore/build/bt-hwsim-usecases-ble-ip-ping-final/run-results.json`
- roles: `ble1`, `ble2`
- datapath: `bt0`, `fc00::1 <-> fc00::2`, `ping6 -c 2`, `0% packet loss`

BLE 6LoWPAN/IPSP 证据：两端状态包含 `linux-bt-6lowpan: registered=1 ifname=bt0`、`ipsp-state=open`、`upstream-owner=net_bluetooth/6lowpan+sim-ipsp-datapath-owner`、`upstream-iphc-owner=net_6lowpan/iphc`，ping 后 `upstream-owner-xmit/rx-deliver/bt-xmit/recv-cb` 和 `tx-iphc/rx-iphc` 均有非零计数，down 后回到 `registered=0`、`ipsp-state=closed`、owner refs 为 0。

BT/BLE NET current closeout：PASS

- case: `bluez-net-current-complete-closeout`
- build: `FeatherCore/build/logs/build-bt1-net-current-final.log`
- build: `FeatherCore/build/logs/build-bt2-net-current-final.log`
- run: `FeatherCore/build/logs/run-net-current-final.log`
- manifest: `FeatherCore/build/bt-hwsim-usecases-net-current-final/run-results.json`
- roles: `bt1`, `bt2`, `ble1`, `ble2`

BT BNEP/BlueZ Network 证据：BT1/BT2 两端通过 `blueznetwork daemon-profile` 覆盖 PANU/NAP/GN role lifecycle，`btn0` 上完成 `10.77.0.1 <-> 10.77.0.2` ping，包含 1400-byte payload ping，`0% packet loss`。日志中 native BNEP path 有 `bnep-native-session-thread`、`bnep-native-kthread-run`、`bnep-native-ndo-start-xmit`、`bnep-native-netdev-xmit`、`bnep-native-tx-frame-ok`、`bnep-native-l2cap-delivered`、`bnep-native-rx-frame-ok`、`bnep-native-netif-rx` 非零计数，且 `bnep-staging-active=0`、cleanup 后 `bnep-native-active=0`。

BlueZ-facing 边界：BT Network coverage map 仍标记 `staged-boundary=blueznetwork-adapter-not-unmodified-bluetoothd`，BLE IPSP coverage map 仍标记 `staged-boundary=bluezdaemon-ipsp-adapter-not-unmodified-bluetoothd`。因此当前结论是：BT/BLE NET 当前 hwsim semantic closeout 已通过，BLE IP ping 旧缺口已收口，但仍不是 unmodified upstream `bluetoothd` Network/IPSP plugin 完全无 adapter 运行。下一步若继续 NET，应优先验证或修复 BT/BLE iperf TCP/UDP matrix 和更长时间生命周期；若回到“完整移植”目标，则要拆除 `blueznetwork`/`bluezdaemon ipsp` staged adapter，改为 upstream daemon/profile 入口直接拥有 fd、D-Bus object 和 session lifecycle。

## 2026-06-14 A2DP upstream convergence baseline + CMake apps entry closeout

本轮在 LE Audio final closeout 和 BT/BLE basic closeout 之后，按顺序重新验证 A2DP 当前最强 upstream convergence gate，并补齐 BlueZ apps 侧 CMake 注册缺口。

A2DP upstream convergence：PASS

- case: `bluez-a2dp-upstream-convergence-closeout`
- build: `FeatherCore/build/logs/build-bt1-a2dp-upstream-final.log`
- build: `FeatherCore/build/logs/build-bt2-a2dp-upstream-final.log`
- run: `FeatherCore/build/logs/run-a2dp-upstream-final.log`
- manifest: `FeatherCore/build/bt-hwsim-usecases-a2dp-upstream-final/run-results.json`
- roles: `bt1` source, `bt2` sink

验证证据包括：A2DP profile/device/SDP lifecycle、D-Bus object lifecycle、mainloop watch/dispatch lifecycle、AVDTP transaction/state machine、MediaTransport fd ownership、SBC codec datapath、AVRCP control/browsing、L2CAP channel ownership、error policy 和两轮 cleanup；source/sink 两端均输出 `final-ok=1`。

构建入口收口：`FeatherCore/apps/wireless/linux_bluetooth/CMakeLists.txt` 已补齐 `CONFIG_LINUX_BLUEZ_NETWORK`、`CONFIG_LINUX_BLUEZ_DAEMON`、`CONFIG_LINUX_BLUEZ_AUDIO` 三个 apps 注册项，并为 `bluezaudio` 补上 SBC/LC3 backend 源与 include flags，使 BlueZ-facing network/daemon/audio apps 不再只覆盖 Makefile 构建路径。

仍保留边界：A2DP gate 明确包含 `staged-boundary=bluezdaemon-adapter-not-unmodified-bluetoothd`。这说明当前 A2DP 已有强 hwsim semantic convergence baseline，但还不是 unmodified upstream `bluetoothd` audio plugin 在 NuttX 上完全无 adapter 运行。下一步进入 BT/BLE NET 前，A2DP 的真实剩余工作是拆除 `bluezdaemon` staged adapter，改为 upstream `bluetoothd` mainloop/D-Bus/profile/audio plugin 入口直接拥有对象和 fd lifecycle。

## 2026-06-14 LE Audio final closeout + BT/BLE basic closeout

本轮按优先级重新收口两个阶段：先验证 LE Audio 当前阶段全功能 umbrella，再验证 BT/BLE 基础能力四角色 upstream convergence gate。

LE Audio final closeout：PASS

- case: `bluez-le-audio-umbrella`
- build: `FeatherCore/build/logs/build-ble1-le-audio-final-closeout.log`
- build: `FeatherCore/build/logs/build-ble2-le-audio-final-closeout.log`
- run: `FeatherCore/build/logs/run-le-audio-final-closeout.log`
- manifest: `FeatherCore/build/bt-hwsim-usecases-le-audio-final-closeout/run-results.json`
- roles: `ble1`, `ble2`

BT/BLE basic closeout：PASS

- case: `bluez-basic-upstream-convergence-closeout`
- build: `FeatherCore/build/logs/build-bt1-basic-final-closeout.log`
- build: `FeatherCore/build/logs/build-bt2-basic-final-closeout.log`
- run: `FeatherCore/build/logs/run-basic-final-closeout.log`
- manifest: `FeatherCore/build/bt-hwsim-usecases-basic-final-closeout/run-results.json`
- roles: `bt1`, `bt2`, `ble2`, `ble1`

当前结论：LE Audio 当前阶段 full-role umbrella 已重新验证通过，BT/BLE 基础扫描、连接、认证、错误路径和基础 BR/EDR/BLE 链路也已四角色通过。下一阶段按顺序继续补完整 A2DP，再补 BT/BLE NET；仍保留 staged adapter 边界说明，不声明 unmodified upstream `bluetoothd` / kernel Bluetooth stack 已完全无适配运行。

# 2026-06-14 BLE Ranging/RAP profile closeout

新增 BLE 双角色 Ranging/RAP profile gate：

```text
case:     bluez-ranging-profile-closeout
roles:    ble1, ble2
build:    build/logs/build-ble1-ranging-profile.log
build:    build/logs/build-ble2-ranging-profile.log
run:      build/logs/run-bluez-ranging-profile-closeout.log
manifest: build/bt-hwsim-usecases-bluez-ranging-profile-closeout/run-results.json
result:   PASS
```

该 gate 要求：

- Ranging initiator/reflector：capability、security、procedure config/start/request、result/event stream、error recovery、cleanup。
- `bluezdaemon profile-ranging-closeout` 输出 BlueZ Ranging/RAP source map 与 Linux HCI/mgmt/L2CAP/SMP source map，且 `final-ok=1`。

边界：仍保留 `staged-boundary=bluezdaemon-ranging-adapter-not-unmodified-bluetoothd`。这是剩余 profile 阶段的第十三个 hwsim semantic closeout，不是 unmodified upstream `bluetoothd` Ranging profile 的最终完成声明。

下一步：进入更深 upstream replacement / 去 staged adapter 化。

# 2026-06-14 BLE MIDI profile closeout

新增 BLE 双角色 MIDI profile gate：

```text
case:     bluez-midi-profile-closeout
roles:    ble1, ble2
build:    build/logs/build-ble1-midi-profile.log
build:    build/logs/build-ble2-midi-profile.log
run:      build/logs/run-bluez-midi-profile-closeout.log
manifest: build/bt-hwsim-usecases-bluez-midi-profile-closeout/run-results.json
result:   PASS
```

该 gate 要求：

- BLE MIDI controller/peripheral：MIDI GATT service、MIDI I/O characteristic、timestamped packets、notify/write、jitter/error policy、cleanup。
- `bluezdaemon profile-midi-closeout` 输出 BlueZ MIDI/GATT source map 与 Linux HCI/mgmt/L2CAP/SMP source map，且 `final-ok=1`。

边界：仍保留 `staged-boundary=bluezdaemon-midi-adapter-not-unmodified-bluetoothd`。这是剩余 profile 阶段的第十二个 hwsim semantic closeout，不是 unmodified upstream `bluetoothd` MIDI profile 的最终完成声明。

下一步：继续 Ranging 等剩余小项，然后推进更深 upstream replacement。

# 2026-06-14 Classic iAP accessory profile closeout

新增 BT 双角色 iAP accessory profile gate：

```text
case:     bluez-iap-profile-closeout
roles:    bt1, bt2
build:    build/logs/build-bt1-iap-profile.log
build:    build/logs/build-bt2-iap-profile.log
run:      build/logs/run-bluez-iap-profile-closeout.log
manifest: build/bt-hwsim-usecases-bluez-iap-profile-closeout/run-results.json
result:   PASS
```

该 gate 要求：

- iAP controller/accessory：SDP/SPP、RFCOMM session、identify、external accessory session、control payload、link control、error recovery、cleanup。
- `bluezdaemon profile-iap-closeout` 输出 BlueZ iAP source map 与 Linux RFCOMM/L2CAP source map，且 `final-ok=1`。

边界：仍保留 `staged-boundary=bluezdaemon-iap-adapter-not-unmodified-iapd`。这是剩余 profile 阶段的第十一个 hwsim semantic closeout，不是 unmodified upstream `iapd` 的最终完成声明。

下一步：继续 MIDI/Ranging 等剩余小项，然后推进更深 upstream replacement。

# 2026-06-14 Classic CUPS/HCRP/SPP printing profile closeout

新增 BT 双角色 CUPS/HCRP/SPP printing profile gate：

```text
case:     bluez-print-profile-closeout
roles:    bt1, bt2
build:    build/logs/build-bt1-print-profile.log
build:    build/logs/build-bt2-print-profile.log
run:      build/logs/run-bluez-print-profile-closeout.log
manifest: build/bt-hwsim-usecases-bluez-print-profile-closeout/run-results.json
result:   PASS
```

该 gate 要求：

- Printing client/printer：SDP HCRP/SPP、RFCOMM session、HCRP control/data、CUPS backend、print job submit/receive/render/status/cancel/error、cleanup。
- `bluezdaemon profile-print-closeout` 输出 BlueZ CUPS/HCRP/SPP source map 与 Linux RFCOMM/L2CAP source map，且 `final-ok=1`。

边界：仍保留 `staged-boundary=bluezdaemon-print-adapter-not-unmodified-cups-backend`。这是剩余 profile 阶段的第十个 hwsim semantic closeout，不是 unmodified upstream CUPS backend/HCRP 的最终完成声明。

下一步：继续 iAP/MIDI/Ranging 等剩余小项，然后推进更深 upstream replacement。

# 2026-06-14 Classic OBEX BIP/Imaging profile closeout

新增 BT 双角色 OBEX BIP/Imaging profile gate：

```text
case:     bluez-obex-bip-profile-closeout
roles:    bt1, bt2
build:    build/logs/build-bt1-obex-bip-profile.log
build:    build/logs/build-bt2-obex-bip-profile.log
run:      build/logs/run-bluez-obex-bip-profile-closeout.log
manifest: build/bt-hwsim-usecases-bluez-obex-bip-profile-closeout/run-results.json
result:   PASS
```

该 gate 要求：

- BIP client/server：SDP、RFCOMM transport、OBEX session、image capabilities、put/get image、thumbnail、abort/error、cleanup。
- `bluezdaemon profile-bip-closeout` 输出 BlueZ `obexd` BIP source map 与 Linux RFCOMM/L2CAP source map，且 `final-ok=1`。

边界：仍保留 `staged-boundary=bluezdaemon-bip-obex-adapter-not-unmodified-obexd`。这是剩余 profile 阶段的第九个 hwsim semantic closeout，不是 unmodified upstream `obexd` BIP 的最终完成声明。

下一步：继续剩余小项，然后推进更深 upstream replacement。

# 2026-06-14 BLE ASHA/Hearing Aid profile closeout

新增 BLE 双角色 ASHA/Hearing Aid profile gate：

```text
case:     bluez-asha-profile-closeout
roles:    ble1, ble2
build:    build/logs/build-ble1-asha-profile.log
build:    build/logs/build-ble2-asha-profile.log
run:      build/logs/run-bluez-asha-profile-closeout.log
manifest: build/bt-hwsim-usecases-bluez-asha-profile-closeout/run-results.json
result:   PASS
```

该 gate 要求：

- ASHA central/hearing-aid：service discovery、GATT control/status、paired side/hi-sync-id、G.722 stream、payload/status、volume/suspend/resume/stop、battery、error/reconnect、cleanup。
- `bluezdaemon profile-asha-closeout` 输出 BlueZ ASHA/audio/GATT/BAS source map 与 Linux HCI/mgmt/L2CAP/SMP source map，且 `final-ok=1`。

边界：仍保留 `staged-boundary=bluezdaemon-asha-adapter-not-unmodified-bluetoothd`。这是剩余 profile 阶段的第八个 hwsim semantic closeout，不是 unmodified upstream `bluetoothd` ASHA profile 的最终完成声明。

下一步：继续剩余小项，然后推进更深 upstream replacement。

# 2026-06-14 Generic BLE GATT/Application Services closeout

新增 BLE 双角色 Generic GATT/Application Services gate：

```text
case:     bluez-gatt-profile-closeout
roles:    ble1, ble2
build:    build/logs/build-ble1-gatt-profile.log
build:    build/logs/build-ble2-gatt-profile.log
run:      build/logs/run-bluez-gatt-profile-closeout.log
manifest: build/bt-hwsim-usecases-bluez-gatt-profile-closeout/run-results.json
result:   PASS
```

该 gate 要求：

- GATT application/database：GAP/BAS/DIS/SCPP/custom service、characteristic、descriptor lifecycle。
- ATT/GATT：MTU/security、discovery、read/write、prepare/execute write、notify/indicate、CCC、long/offset read、error policy、reconnect/unregister、cleanup。
- `bluezdaemon profile-gatt-closeout` 输出 BlueZ GATT database/client/shared/profile source map 与 Linux HCI/mgmt/L2CAP/SMP source map，且 `final-ok=1`。

边界：仍保留 `staged-boundary=bluezdaemon-gatt-adapter-not-unmodified-bluetoothd`。这是剩余 profile 阶段的第七个 hwsim semantic closeout，不是 unmodified upstream `bluetoothd` GATT database/client 的最终完成声明。

下一步：继续剩余 profile 小项，然后推进更深 upstream replacement。

# 2026-06-14 BLE Mesh profile closeout

新增 BLE 双角色 Mesh profile gate：

```text
case:     bluez-mesh-profile-closeout
roles:    ble1, ble2
build:    build/logs/build-ble1-mesh-profile.log
build:    build/logs/build-ble2-mesh-profile.log
run:      build/logs/run-bluez-mesh-profile-closeout.log
manifest: build/bt-hwsim-usecases-bluez-mesh-profile-closeout/run-results.json
result:   PASS
```

该 gate 要求：

- Provisioner/node：daemon init、ADV/GATT Proxy bearer、PB-ADV provisioning、key material、config client/server、Generic OnOff model。
- Mesh networking：transport segmentation/reassembly、relay、friend/LPN、proxy、beacon、heartbeat、RPL/replay protection、error recovery、cleanup。
- `bluezdaemon profile-mesh-closeout` 输出 BlueZ `mesh` source map 与 Linux HCI/mgmt/L2CAP/SMP source map，且 `final-ok=1`。

边界：仍保留 `staged-boundary=bluezdaemon-mesh-adapter-not-unmodified-bluetoothd`。这是剩余 profile 阶段的第六个 hwsim semantic closeout，不是 unmodified upstream `bluetooth-meshd` 的最终完成声明。

下一步：继续 Generic GATT/Application services 等剩余 profiles，然后推进更深 upstream replacement。

# 2026-06-14 Classic OBEX FTP/Sync profile closeout

新增 BT 双角色 OBEX FTP/Sync profile gate：

```text
case:     bluez-obex-ftp-sync-profile-closeout
roles:    bt1, bt2
build:    build/logs/build-bt1-obex-ftp-sync-profile.log
build:    build/logs/build-bt2-obex-ftp-sync-profile.log
run:      build/logs/run-bluez-obex-ftp-sync-profile-closeout.log
manifest: build/bt-hwsim-usecases-bluez-obex-ftp-sync-profile-closeout/run-results.json
result:   PASS
```

该 gate 要求：

- FTP client/server：SDP、RFCOMM transport、OBEX session、folder listing、get/put/delete file、abort/error、cleanup。
- Sync client/server：SDP、RFCOMM transport、OBEX session、phonebook/calendar/notes sync、abort/error、cleanup。
- `bluezdaemon profile-sync-closeout` 输出 BlueZ `obexd` FTP/Sync source map 与 Linux RFCOMM/L2CAP source map，且 `final-ok=1`。

边界：仍保留 `staged-boundary=bluezdaemon-ftp-obex-adapter-not-unmodified-obexd` 与 `staged-boundary=bluezdaemon-sync-obex-adapter-not-unmodified-obexd`。这是剩余 profile 阶段的第五个 hwsim semantic closeout，不是 unmodified upstream `obexd` 的最终完成声明。

下一步：继续 Generic GATT、Mesh 等剩余 profiles，然后推进更深 upstream replacement。

# 2026-06-14 Classic OBEX MAP/MNS profile closeout

新增 BT 双角色 OBEX MAP/MNS profile gate：

```text
case:     bluez-obex-map-mns-profile-closeout
roles:    bt1, bt2
build:    build/logs/build-bt1-obex-map-mns-profile.log
build:    build/logs/build-bt2-obex-map-mns-profile.log
run:      build/logs/run-bluez-obex-map-mns-profile-closeout.log
manifest: build/bt-hwsim-usecases-bluez-obex-map-mns-profile-closeout/run-results.json
result:   PASS
```

该 gate 要求：

- MAP MAS/client-server：SDP、RFCOMM transport、OBEX session、set-folder、message listing/get/status/push、abort/error、cleanup。
- MNS client/server：SDP、RFCOMM transport、OBEX session、NewMessage/DeliverySuccess/MessageDeleted event report、abort/error、cleanup。
- `bluezdaemon profile-map-closeout` 输出 BlueZ `obexd` MAP/MNS source map 与 Linux RFCOMM/L2CAP source map，且 `final-ok=1`。

边界：仍保留 `staged-boundary=bluezdaemon-map-obex-adapter-not-unmodified-obexd` 与 `staged-boundary=bluezdaemon-mns-obex-adapter-not-unmodified-obexd`。这是剩余 profile 阶段的第四个 hwsim semantic closeout，不是 unmodified upstream `obexd` 的最终完成声明。

下一步：继续 FTP/Sync、Generic GATT、Mesh 等剩余 profiles，然后推进更深 upstream replacement。

# 2026-06-14 Classic OBEX PBAP/OPP profile closeout

新增 BT 双角色 OBEX PBAP/OPP profile gate：

```text
case:     bluez-obex-pbap-opp-profile-closeout
roles:    bt1, bt2
build:    build/logs/build-bt1-obex-pbap-opp-profile.log
build:    build/logs/build-bt2-obex-pbap-opp-profile.log
run:      build/logs/run-bluez-obex-pbap-opp-profile-closeout.log
manifest: build/bt-hwsim-usecases-bluez-obex-pbap-opp-profile-closeout/run-results.json
result:   PASS
```

该 gate 要求：

- PBAP PCE/PSE：SDP、RFCOMM transport、OBEX session、phonebook select、pull phonebook/listing/entry、abort/error、cleanup。
- OPP client/server：SDP、RFCOMM transport、OBEX session、object push、capability query、abort/error、cleanup。
- `bluezdaemon profile-obex-closeout` 输出 BlueZ `obexd` core/client/plugins source map 与 Linux RFCOMM/L2CAP source map，且 `final-ok=1`。

边界：仍保留 `staged-boundary=bluezdaemon-pbap-obex-adapter-not-unmodified-obexd` 与 `staged-boundary=bluezdaemon-opp-obex-adapter-not-unmodified-obexd`。这是剩余 profile 阶段的第三个 hwsim semantic closeout，不是 unmodified upstream `obexd` 的最终完成声明。

下一步：继续 MAP/MNS、FTP/Sync、Generic GATT、Mesh 等剩余 profiles，然后推进更深 upstream replacement。

# 2026-06-14 Classic HFP/HSP profile closeout

新增 BT 双角色 headset/telephony profile gate：

```text
case:     bluez-hfp-hsp-profile-closeout
roles:    bt1, bt2
build:    build/logs/build-bt1-hfp-hsp-profile.log
build:    build/logs/build-bt2-hfp-hsp-profile.log
run:      build/logs/run-bluez-hfp-hsp-profile-closeout.log
manifest: build/bt-hwsim-usecases-bluez-hfp-hsp-profile-closeout/run-results.json
result:   PASS
```

该 gate 要求：

- HFP HF/AG：SDP service、Profile1-style connect、RFCOMM session、AT SLC、indicator/call-control、codec negotiation、SCO audio、volume、cleanup。
- HSP HS/AG：SDP service、RFCOMM session、CKPD/RING/VGS/VGM control、SCO audio、cleanup。
- `bluezdaemon profile-hfp-closeout` 输出 BlueZ HFP/telephony/media/transport source map 与 Linux RFCOMM/SCO/L2CAP source map，且 `final-ok=1`。

边界：仍保留 `staged-boundary=bluezdaemon-hfp-adapter-not-unmodified-bluetoothd` 与 `staged-boundary=bluezdaemon-hsp-adapter-not-unmodified-bluetoothd`。这是剩余 profile 阶段的第二个 hwsim semantic closeout，不是 unmodified upstream `bluetoothd` HFP/HSP 的最终完成声明。

下一步：继续 PBAP/OBEX、MAP、Generic GATT、Mesh 等剩余 profiles，然后推进更深 upstream replacement。

# 2026-06-14 Classic HID + BLE HOGP profile closeout

新增四角色 input profile gate：

```text
case:     bluez-hid-hogp-profile-closeout
roles:    bt1, bt2, ble2, ble1
build:    build/logs/build-bt1-hid-hogp-profile.log
build:    build/logs/build-bt2-hid-hogp-profile.log
build:    build/logs/build-ble1-hid-hogp-profile.log
build:    build/logs/build-ble2-hid-hogp-profile.log
run:      build/logs/run-bluez-hid-hogp-profile-closeout.log
manifest: build/bt-hwsim-usecases-bluez-hid-hogp-profile-closeout/run-results.json
result:   PASS
```

该 gate 要求：

- BT1/BT2 Classic HID：SDP HID service、Profile1-style connect、L2CAP control PSM `0x0011`、interrupt PSM `0x0013`、HIDP connadd、input/output report、cleanup。
- BLE2/BLE1 HOGP：GATT HID service、report map、protocol mode、boot reports、CCC notify、input/output report、suspend/resume、cleanup。
- `bluezdaemon profile-hid-closeout` 输出 BlueZ input/HOGP/profile/device/adapter/shared GATT source map 与 Linux HIDP/L2CAP/SMP source map，且 `final-ok=1`。

边界：仍保留 `staged-boundary=bluezdaemon-input-adapter-not-unmodified-bluetoothd` 与 `staged-boundary=bluezdaemon-hogp-adapter-not-unmodified-bluetoothd`。这是剩余 profile 阶段的第一个 hwsim semantic closeout，不是 unmodified upstream `bluetoothd` input plugin 的最终完成声明。

下一步：继续剩余 BT/BLE profiles，然后推进更深 upstream replacement。

# 2026-06-14 BT/BLE NET upstream convergence rerun after A2DP

BT/BLE basic 与 A2DP upstream convergence 都通过后，重新运行四角色 NET gate：

```text
case:     bluez-net-upstream-convergence-closeout
roles:    bt1, bt2, ble1, ble2
run:      build/logs/run-bluez-net-upstream-convergence-after-a2dp.log
manifest: build/bt-hwsim-usecases-bluez-net-upstream-convergence-after-a2dp/run-results.json
result:   PASS
```

该 gate 继续要求：

- BT Network/BNEP：PANU/NAP/GN role policy、NetworkServer1 registration、L2CAP fd handoff、BNEPCONNADD、btn0 ping、MTU1400 ping、BNEP native datapath counters、disconnect/unregister cleanup。
- BLE IPSP/6LoWPAN：Profile1/mainloop、LE L2CAP CoC fd handoff、bt0 ping6、TCP/UDP iperf、IPHC/fragment counters、disconnect cleanup、owner refs 归零。
- `bluez-network: closeout upstream-coverage-map ... final-ok=1` 和 `bluez-daemon: ipsp closeout upstream-coverage-map ... final-ok=1`。

边界：仍保留 `staged-boundary=blueznetwork-adapter-not-unmodified-bluetoothd` 与 `staged-boundary=bluezdaemon-ipsp-adapter-not-unmodified-bluetoothd`。这是 NET 当前 hwsim semantic closeout 证据，不是 unmodified upstream `bluetoothd` Network plugin 与 Linux kernel BNEP/6LoWPAN 完全无 adapter harness 的声明。

下一步：进入剩余 BT/BLE profile 和更深 upstream replacement。

# 2026-06-14 A2DP upstream convergence rerun after BT/BLE basic

`bluez-basic-upstream-convergence-closeout` 通过后，重新运行 A2DP upstream convergence gate：

```text
case:     bluez-a2dp-upstream-convergence-closeout
roles:    bt1, bt2
run:      build/logs/run-bluez-a2dp-upstream-convergence-after-basic.log
manifest: build/bt-hwsim-usecases-bluez-a2dp-upstream-convergence-after-basic/run-results.json
result:   PASS
```

该 gate 继续要求 source/sink 双端的 `audio-a2dp-closeout-full`，包括 profile/session、SDP profile、D-Bus object lifecycle、mainloop/watch ownership、AVDTP transaction/state machine、AVRCP、MediaTransport fd、SBC codec、L2CAP controller ownership、error policy 和 final cleanup。

边界：仍保留 `staged-boundary=bluezdaemon-adapter-not-unmodified-bluetoothd`。这是 A2DP 当前 hwsim semantic closeout 证据，不是 unmodified upstream `bluetoothd` audio plugin 完全直接运行声明。

下一步：继续 BT/BLE NET completion。

# 2026-06-14 BT/BLE basic upstream convergence closeout

新增四角色 basic closeout gate：

```text
case:     bluez-basic-upstream-convergence-closeout
roles:    bt1, bt2, ble2, ble1
build:    build/logs/build-bt1-basic-upstream-convergence.log
build:    build/logs/build-bt2-basic-upstream-convergence.log
build:    build/logs/build-ble1-basic-upstream-convergence.log
build:    build/logs/build-ble2-basic-upstream-convergence.log
run:      build/logs/run-bluez-basic-upstream-convergence-closeout.log
manifest: build/bt-hwsim-usecases-bluez-basic-upstream-convergence-closeout/run-results.json
result:   PASS
```

该 gate 将 BT/BLE basic 能力从分散验证收束成一个阶段边界：

- BT1/BT2：BR/EDR mgmt power/connectable/discoverable、scan/connect/pair、L2CAP echo、basic closeout。
- BLE2：LE power、advertising、重复 advertising 窗口、basic closeout。
- BLE1：BlueZ daemon discovery-peer、btmgmt control/lifecycle/pair-noio、daemon pairing-matrix、reconnect-stress、device-policy、btmgmt error-path、basic closeout。
- `bluezdaemon basic-closeout bt|ble` 输出 `basic closeout upstream-coverage-map ... final-ok=1`，要求 BlueZ `main.c`、`adapter.c`、`device.c`、`agent.c`、`dbus-common.c`、`shared/mainloop.c`、`shared/io-mainloop.c`、`btmgmt.c`、`hcitool.c` 与 Linux `hci_core.c`、`hci_sock.c`、`mgmt.c`、`l2cap_core.c`、`l2cap_sock.c`、`smp.c` 出现在 evidence 中。

边界：该 gate 是当前 NuttX sim hwsim 的 BT/BLE basic semantic closeout，保留 `staged-boundary=bluezdaemon-basic-adapter-not-unmodified-bluetoothd`。它证明基础扫描、连接、认证、重连、错误路径和 BR/EDR basic L2CAP link 已有闭环证据，但不是整套 Linux 蓝牙协议栈完成声明。

下一步：按当前顺序回到 A2DP completion，再继续 BT/BLE NET completion。

# 2026-06-14 BLE GATT/ATT upstream convergence closeout

新增 BLE GATT/ATT completion gate：

```text
case:     bluez-gatt-upstream-convergence-closeout
roles:    ble1, ble2
build:    build/logs/build-ble1-gatt-upstream-convergence.log
build:    build/logs/build-ble2-gatt-upstream-convergence.log
run:      build/logs/run-bluez-gatt-upstream-convergence-closeout.log
manifest: build/bt-hwsim-usecases-bluez-gatt-upstream-convergence-closeout/run-results.json
result:   PASS
```

该 gate 让 BLE GATT/ATT 不再只作为 LE Audio 子步骤隐式出现，而是单独要求 source/sink 双角色完成：

- ATT bearer open、MTU exchange、security、CCC、prepare/execute write、indication、close。
- ATT I/O attach、mainloop rx/tx watch、rx/tx PDU、fragment/reassemble、CCC persist、flush、detach。
- ATT request queue alloc、enqueue、socket read/write、timeout、cancel、error response、complete、free。
- PACS/ASCS GATT DB register、discover、read、write、notify、release。
- `le-gatt upstream-coverage-map ... final-ok=1`，并要求 BlueZ `shared/att.c`、`gatt-db.c`、`gatt-server.c`、`io-mainloop.c`、audio `pacs.c`、`ascs.c` 与 Linux `l2cap_sock.c`、`l2cap_core.c` 出现在 evidence 中。

边界：当前仍是 hwsim semantic closeout，保留 `staged-boundary=bluezaudio-gatt-adapter-not-unmodified-bluetoothd`。它用于收口 BLE 基础能力和 LE Audio 共用的 ATT/GATT 地基，不代表 unmodified upstream `bluetoothd` 已完整直接运行。

后续顺序固定为：LE Audio 全功能 -> BT/BLE basic -> A2DP -> BT/BLE NET -> 剩余 BT/BLE profile。

# 2026-06-14 A2DP current complete closeout

新增 A2DP completion gate：

```text
case:     bluez-a2dp-current-complete-closeout
roles:    bt1, bt2
run:      build/logs/run-bluez-a2dp-current-complete-closeout.log
manifest: build/bt-hwsim-usecases-bluez-a2dp-current-complete-closeout/run-results.json
result:   PASS
```

该 gate 固化 `bluezdaemon audio-a2dp-closeout-full source|sink` 为当前 A2DP 完成态，覆盖 bluetoothd-shaped mainloop、SDP/profile、D-Bus owner recovery、AVDTP/AVRCP、MediaTransport fd、SBC codec、controller-created L2CAP、error policy、两轮 lifecycle 和 final cleanup。

# 2026-06-14 BlueZ HCI/mgmt/socket ABI closeout

新增 BLE 双角色基础 ABI gate：

```text
case:     bluez-hci-mgmt-socket-closeout-full
roles:    ble1, ble2
run:      build/logs/run-bluez-hci-mgmt-socket-closeout-full.log
manifest: build/bt-hwsim-usecases-bluez-hci-mgmt-socket-closeout-full/run-results.json
result:   PASS
```

覆盖点：

- BLE1: HCI USER advertising、command、monitor、sequence、error、init sequence、ISO setup monitor。
- BLE2: HCI USER scan report，消费 BLE1 写入的 hwsim ADV public-file medium。
- BLE1: HCI ioctl basic、HCI RAW command、BlueZ mgmt control/pairing/error/reconnect-stress、btmon monitor。

语义约束：`HCI_CHANNEL_USER` 按 Linux 行为是独占 channel，必须在 ioctl/mgmt 接管 controller 前执行；否则会得到 busy。该 gate 已按 USER -> ioctl/raw/mgmt/monitor 的顺序固定。

# 2026-06-14 BlueZ current functional closeout

新增 hwsim 组合 gate：

```text
case:     bluez-current-functional-closeout
roles:    bt1, bt2, ble1, ble2
run:      build/logs/run-bluez-current-functional-closeout.log
manifest: build/bt-hwsim-usecases-bluez-current-functional-closeout/run-results.json
result:   PASS
```

该 gate 同时启动四个 sim 角色：

- BT1/BT2 先执行 A2DP daemon closeout，再执行 BlueZ Network/BNEP closeout。
- BLE1/BLE2 同时执行 bluetoothd-style IPSP closeout。
- validator 检查 A2DP complete/cleanup、BT Network `bnep-native-active=0`、BLE IPSP `registered=0` / `ipsp-state=closed`。

这轮先暴露了 A2DP source transaction 在繁忙 hwsim 下可能漏记 `avdtp-start` response 的问题；修复为 expected-payload 匹配后通过。

# 2026-06-14 LE Audio umbrella 与 BT/BLE basic closeout

当前 hwsim 验证顺序先完成 LE Audio full-role umbrella，再完成 BT/BLE 基础能力 closeout。

LE Audio gate：

```text
case:     bluez-le-audio-umbrella
run:      build/logs/run-le-audio-umbrella.log
manifest: build/bt-hwsim-usecases-le-audio-umbrella/run-results.json
result:   PASS
```

BT/BLE basic gate：

```text
cases:
  bt-basic
  ble-basic
  hci-le-lifecycle
  hci-le-medium
  hci-le-pairing
  mgmt-control
  bluez-mgmt-control
  bluez-mgmt-pair-noio
  bluez-mgmt-pair-unpair
  bluez-mgmt-lifecycle
  bluez-daemon-mgmt-full-lifecycle
  bluez-basic-mgmt-flow
  bluez-basic-scan-connect-auth-flow

run:      build/logs/run-basic-closeout.log
manifest: build/bt-hwsim-usecases-basic-closeout/run-results.json
result:   PASS
```

`btctl poll ctrl` 语义更新：

- 读取 `bt-hwsim-ctrl.bin` 的 raw CTRL record。
- 跳过本机 self record，只处理 peer/control broadcast record。
- 调用 Linux-port control record processor，更新本地 connect/disconnect/pairing state。
- 非阻塞返回；没有新 record 时返回 0。
- 用于 passive BLE 端在不阻塞 scan 的情况下推进 HCI connection lifecycle，并配合 `btctl events` / `btctl upstream hci-status` 输出 validator evidence。

边界：这些 gate 证明当前 sim/hwsim 通路已经收口，但完整 upstream Linux Bluetooth host stack 接管仍按 staged Kconfig 继续推进。

# NuttX sim BT/BLE hwsim 移植入口

## 当前落地内容

本目录新增 4 个和 Wi-Fi hwsim 角色一致的构建入口：

```bash
tools/firmware/sim/build-bt1.sh
tools/firmware/sim/build-bt2.sh
tools/firmware/sim/build-ble1.sh
tools/firmware/sim/build-ble2.sh
```

它们分别构建以下独立 defconfig：

```text
sim:hwsim_bt1
sim:hwsim_bt2
sim:hwsim_ble1
sim:hwsim_ble2
```

输出文件沿用 Wi-Fi hwsim 约定：

```text
build/nuttx-sim-bt1
build/nuttx-sim-bt2
build/nuttx-sim-ble1
build/nuttx-sim-ble2
```

同时新增 Linux-port 专用 nsh 命令骨架：

```text
btctl
btaudio
```

这两个命令已经进入四套 defconfig，并通过 `linux_bt_*` 语义 API 进入
Linux-port core。当前必须遵守 upstream-first 规则：能从 Linux
`net/bluetooth` 直接移植的协议逻辑优先直接移植；只有 NuttX/Unix 兼容缺口
才放入 compat/shim。core 当前会把 HCI/L2CAP/GATT/A2DP/ISO 语义动作编码成
`SIM_BTHWSIM` 公共文件介质里的 synthetic records，用来先验证四个独立
sim 终端之间的 hwsim 数据路径。第一阶段已经具备 Linux-like HCI control、
ACL/L2CAP、SMP、ATT/GATT、LE advertising、A2DP media、ISO record 的端到端
注入/读取能力，并新增最小 `hci_conn`、`l2cap_chan`、`att_db` 风格对象表、
`iso_path` 风格 BIG/BIS 表、本地 HCI event ring 和最小 Linux mgmt
controller settings；完整 Linux `net/bluetooth` 后端正在按 staged Kconfig
逐步接入。

## 语义目标

最终目标不是简单转发 H4 字节流，也不是从零重写 NuttX 原生蓝牙，而是直接移植
Linux Bluetooth host/controller 语义：

```text
NuttX apps / BlueZ-like nsh command
  -> Linux Bluetooth socket / mgmt / profile API surface
  -> imported Linux net/bluetooth host semantics
  -> imported Linux drivers/bluetooth/hci_vhci.c
  -> SIM_BTHWSIM public-file ACL/ADV/ISO medium
  -> peer upstream VHCI write_iter()/hci_recv_frame()
  -> peer imported Linux Bluetooth stack
```

这和 Wi-Fi `mac80211_hwsim` 的文件介质模型对应：

```text
Wi-Fi: hwsim-bss.bin + hwsim-frames.bin
BT:    bt-hwsim-ctrl.bin + bt-hwsim-acl.bin + bt-hwsim-iso.bin
BLE:   bt-hwsim-adv.bin + bt-hwsim-acl.bin + bt-hwsim-iso.bin
```

建议公共文件 record 头：

```c
struct bthwsim_record {
  uint32_t magic;      /* "BTHS" */
  uint16_t version;    /* 1 */
  uint16_t type;       /* EVT, ACL, ISO, ADV, CTRL */
  uint16_t src;        /* bt1/bt2/ble1/ble2 role id */
  uint16_t dst;        /* peer id or broadcast */
  uint32_t seq;
  uint64_t timestamp_us;
  uint32_t payload_len;
  uint32_t crc32;
  uint8_t  payload[];  /* HCI payload or simulated air packet */
};
```

## 当前 defconfig 分层

第一阶段配置不再启用 NuttX 原来的 `wireless/bluetooth`、`BTSAK`、`NimBLE`
或 `SIM_HCISOCKET` 路径，而是建立 Linux-port 专用命名空间：

- `hwsim_bt1/hwsim_bt2` 启用 `NET_LINUX_BLUETOOTH`、`SIM_BTHWSIM`，mode 为 `bredr`。
- `hwsim_ble1/hwsim_ble2` 启用 `NET_LINUX_BLUETOOTH`、`SIM_BTHWSIM`，mode 为 `le`。
- 每个配置设置独立 `CONFIG_LIBC_HOSTNAME` 和 `CONFIG_SIM_BTHWSIM_ROLE`，用于公共文件介质里的源/目的过滤。

`SIM_BTHWSIM` 当前已经创建公共介质文件：

```text
/tmp/nuttx-bthwsim/bt-hwsim-ctrl.bin
/tmp/nuttx-bthwsim/bt-hwsim-adv.bin
/tmp/nuttx-bthwsim/bt-hwsim-acl.bin
/tmp/nuttx-bthwsim/bt-hwsim-iso.bin
```

并为每个 role/type 维护独立读取 offset 文件，同时为 Linux-like 连接语义
维护状态文件：

```text
/tmp/nuttx-bthwsim/linux-bt-conn-<self>-<peer>.state
```

当前连接 handle 使用稳定公式生成，保证两个 sim 端对同一条连接得到一致
handle：

```text
handle = 0x0040 + min(self, peer) * 16 + max(self, peer)
```

后续要在该后端上补齐 Linux HCI controller event、ACL、advertising、ISO/BIS
调度语义。

## 后续驱动实现拆分

1. `CONFIG_SIM_BTHWSIM` 已新增。
2. `arch/sim/src/sim` 已新增 BT hwsim hostfile 后端：
   - 已创建公共介质文件。
   - 已支持 append-only 写入公共介质文件。
   - 已支持每个 role/type 保存独立 read offset。
   - 已支持按 role 跳过自己写入的 record。
   - 已支持 broadcast record 读取。
   - 已支持 raw binary record 读取，并使用独立 `.raw.roleX.off` offset，避免 `btctl poll` 文本调试消费 upstream HCI 注入数据。
   - 已支持 Linux-like connection state 文件。
   - 后续还要补精确 fanout、丢包/延迟/信道策略。
3. NuttX 侧已新增 Linux-port BT core 命名空间和 upstream import 区：
   - Linux kernel 侧分层固定为：`net/bluetooth` 语义进入 `nuttx/wireless/linux_bluetooth`，`drivers/bluetooth` 语义进入 `nuttx/drivers/bluetooth`；`apps/wireless/linux_bluetooth` 只放 nsh/BlueZ-like 用户态验证工具。
   - BT port 的通用 Linux compat 不应重复发明 Wi-Fi/mac80211 已经实现的内容；`wireless/linux_bluetooth` 和 `drivers/bluetooth` 的 Make/CMake 已把 `wireless/ieee80211/include` 作为 fallback include 路径，优先复用已有 `linux/*.h` 兼容层。
   - 上游 Linux 源码已导入 `wireless/linux_bluetooth/upstream/net_bluetooth`。
   - 上游 Linux 头文件已导入 `wireless/linux_bluetooth/upstream/include_net_bluetooth`。
   - 上游 Linux 虚拟 HCI driver 已导入 `wireless/linux_bluetooth/upstream/drivers_bluetooth/hci_vhci.c`。
   - `hci_vhci.c` 的构建归属已迁到 `nuttx/drivers/bluetooth`，保持和 Linux `drivers/bluetooth` 同层；`drivers/bluetooth/linux_bt_upstream_vhci.c` 只保留 NuttX-facing VHCI open/read/write/pump 边界。
   - compat 骨架位于 `wireless/linux_bluetooth/upstream/compat`。
   - compat 当前已补入最低层 `linux/types.h`、`linux/kernel.h`、`linux/list.h`、`linux/slab.h`、`linux/err.h`、`linux/skbuff.h`、`linux/spinlock.h`、`linux/mutex.h`、`linux/atomic.h`、`linux/refcount.h`、`linux/workqueue.h`、`linux/wait.h`、`linux/idr.h`、`linux/rculist.h`、`linux/srcu.h`、`linux/leds.h`、`linux/unaligned.h`、`linux/fs.h`、`linux/miscdevice.h`、`linux/debugfs.h`、`net/sock.h` 等头骨架，并映射 `<net/bluetooth/*.h>` 到导入的上游头文件。
   - compat debugfs attribute 已从空 stub 推进为可读写 `file_operations`；`kstrtobool_from_user()`、`kstrtoull_from_user()` 覆盖 `hci_vhci.c` 的 force_suspend/force_wakeup/msft_opcode/aosp_capable 输入路径。
   - `CONFIG_NET_LINUX_BLUETOOTH_UPSTREAM_SOURCES` 是逐步编译上游文件的总开关，默认关闭，避免未完成 compat 时破坏 sim bring-up。
   - `CONFIG_NET_LINUX_BLUETOOTH_UPSTREAM_LIB` 第一阶段只编 `upstream/net_bluetooth/lib.c`。
   - `CONFIG_NET_LINUX_BLUETOOTH_UPSTREAM_EIR`、`CONFIG_NET_LINUX_BLUETOOTH_UPSTREAM_HCI_CODEC` 和 `CONFIG_NET_LINUX_BLUETOOTH_UPSTREAM_HCI_VHCI` 单独控制，分别等待 Linux `hci_core.h/struct hci_dev/uuid list`、`sk_buff/HCI sync`、`miscdevice/file operation/sk_buff queue` compat 就绪后再启用。
   - `CONFIG_DRIVERS_BLUETOOTH_LINUX_HCI_VHCI` 是 NuttX driver 层的 VHCI 源文件构建开关，默认跟随 `CONFIG_NET_LINUX_BLUETOOTH_UPSTREAM_HCI_VHCI`。
   - `CONFIG_NET_LINUX_BLUETOOTH_UPSTREAM_AF` 当前在四个 hwsim defconfig 默认关闭；Linux 原始 `af_bluetooth.c` 已保留在 upstream import 区和 staged Kconfig 中，但第一阶段不直接编入 sim 镜像，避免它和当前已经可运行的 NuttX-facing `PF_BLUETOOTH`/HCI/L2CAP/ISO shim 双重注册。后续补齐 Linux socket core compat 后，再用 upstream `bt_init()` 接管该层。
   - `CONFIG_NET_LINUX_BLUETOOTH_UPSTREAM_HCI_VHCI_AUTOSTART` 可在 `SIM_BTHWSIM` ready 后自动 open upstream VHCI；四个 hwsim defconfig 已默认打开，使每个 nsh 终端启动时都有 upstream VHCI miscdevice 实例。
   - 已新增 `drivers/bluetooth/linux_bt_upstream_vhci.c` 作为 NuttX-facing VHCI port 边界；`btctl upstream` 可从 nsh 查看当前 upstream VHCI 接入状态。
   - compat 里的 `<net/bluetooth/hci_core.h>` 当前是 VHCI 阶段 wrapper：保留 upstream `hci.h`/`bluetooth.h`，声明 `hci_vhci.c` 所需的 `struct hci_dev`、quirk、register/free、`hci_recv_frame()` 边界。
   - `linux_bt_upstream_hci.c` 维护共享 HCI device registry，避免 header static inline 让每个 upstream 编译单元各自拥有一份假 controller 状态；`btctl upstream` 会输出已注册 `hci_dev`、bus、quirks、rx/tx 计数。
   - `btctl upstream` 还会输出 AF/VHCI 状态：`upstream-af` 行显示 `PF_BLUETOOTH` 是否已经通过 upstream `bt_init()` 注册；VHCI 行里的 `m2h` 表示 medium->host write_iter 次数，`h2m` 表示 host->medium drain 次数，`read-empty` 表示 `vhci_read()` 当前无 H4 数据。
   - compat `module_misc_device(vhci_miscdev)` 现在会生成可调用注册函数；`btctl upstream open` 或 `NET_LINUX_BLUETOOTH_UPSTREAM_HCI_VHCI_AUTOSTART` 会注册 upstream `vhci` miscdevice 并沿 `hci_vhci.c` 自己的 `vhci_fops.open()` 创建默认 VHCI 实例。
   - `hci_recv_frame()` 当前按 Linux VHCI 方向作为 controller/medium -> host 的入口，只做 HCI RX 侧 staging/accounting，不把收到的 packet 回写公共介质。
   - `btctl upstream poll` 会从 raw 公共介质读取 acl/iso record，封装成 H4 packet 后走 upstream `vhci_fops.write_iter()`；完整 Linux `hci_core.c` event queue/socket wakeup 后续再接管。
   - `btctl upstream read` 会走 upstream `vhci_fops.read()`，读取 host->controller 方向排队的 H4 字节流。
   - `btctl upstream drain` 会走 upstream `vhci_fops.read()`，解析 H4 packet type，只把 ACL/ISO host->controller 数据写回 `SIM_BTHWSIM` 公共介质；HCI command/event/vendor 保持在本地 host/controller 控制面，空队列的 `-EAGAIN` 会被视为 drain 完成。
   - `btctl upstream create [opcode]` 会通过 upstream `vhci_fops.write_iter()` 写入 H4 vendor packet，让 imported `hci_vhci.c` 自己执行 `vhci_create_device()`。
   - `btctl upstream pump` 是 `drain` + `poll` 的组合命令，用于两个独立 nsh 终端手动推进公共文件介质。
   - `btctl upstream socket hci|l2cap|iso` 会从用户态触发 upstream `PF_BLUETOOTH` family 的 `create()` 路径，用于观察 BlueZ 继续接入时的 socket 边界。当前 `hci` 已通过 guarded staging `hci_sock_init()` 走 `proto_register()`、注册 `BTPROTO_HCI` 并走 upstream `bt_sock_alloc()`；HCI staging create 也已补 `proto_ops`、`SS_UNCONNECTED` state 和 release op，probe 会输出 `state/ops/sk-state/sk-proto`。`btctl upstream socket hci raw|user|monitor|control|logging [dev]` 会继续触发 staging bind，RAW/USER 绑定到具体 HCI index，CONTROL/MONITOR/LOGGING 绑定到 `HCI_DEV_NONE`，并在绑定具体 hdev 时调用 `getname(peer=0)` 读回 Linux `sockaddr_hci`。`btctl upstream socket-filter <dev> <type-mask> <event-mask0> <event-mask1> [opcode]` 会创建 RAW HCI socket，走 `setsockopt(SOL_HCI, HCI_FILTER)` / `getsockopt(SOL_HCI, HCI_FILTER)`，并让 controller->host RAW fanout 按 packet type、event mask 和 command opcode 过滤。`l2cap` 现在也会通过 guarded staging `l2cap_init()` 注册 `BTPROTO_L2CAP`，并用 upstream `bt_sock_alloc()` 分配 `struct l2cap_pinfo`；`btctl upstream socket l2cap <psm> [cid]` 会构造 imported upstream `struct sockaddr_l2` bind 请求，记录本地 PSM/CID，并把 socket 推进到 `BT_BOUND`。`btctl upstream l2cap-send <psm> <cid> <handle> <hex...>` 会创建并 bind L2CAP socket，通过 socket `sendmsg()` 把 payload 包装成 HCI ACL data + L2CAP basic header，再进入 upstream VHCI send path，用于观察 BlueZ L2CAP/A2DP socket -> kernel -> VHCI 方向。controller/medium -> host 方向也有协议 socket fanout：`hci_recv_frame()` 收到 ACL 后会解析 ACL/L2CAP header，并把 L2CAP payload 按 CID 入队到 matching bound L2CAP socket；收到 ISO 后会解析 HCI ISO header，并把 ISO SDU payload 入队到 bound ISO socket。`btctl upstream l2cap-bind <psm> <cid> <handle>` 和 `btctl upstream iso-bind <addr-type> <handle>` 会在当前 sim 实例里保留一个 staging 协议 socket，便于另一个终端发送、当前终端 pump/poll 后用 `btctl upstream l2cap-recv [max]` 或 `btctl upstream iso-recv [max]` 读取队列 payload。`iso` 现在也会通过 guarded staging `iso_init()` 注册 `BTPROTO_ISO`；`btctl upstream socket iso [addr-type]` 会用 imported upstream `struct sockaddr_iso` bind 请求记录本地地址类型，并把 socket 推进到 `BT_BOUND`。`btctl upstream iso-send <addr-type> <handle> <hex...>` 会创建并 bind ISO socket，通过 socket `sendmsg()` 把 payload 包装成 HCI ISO data packet，再进入 upstream VHCI send path，用于观察 BlueZ ISO socket -> kernel -> VHCI 方向。L2CAP/ISO 的完整 connect/session/reassembly/flow-control 语义仍保持 staging，等待后续启用 upstream L2CAP socket/channel 和 upstream `iso.c` 文件。
   - `btctl upstream mgmt-socket <opcode> [index] [param]` 会创建 upstream `PF_BLUETOOTH/BTPROTO_HCI` socket，bind 到 `HCI_CHANNEL_CONTROL`，再用 Linux `struct mgmt_hdr` 形状走 HCI socket `sendmsg()` staging 边界。当前 `mgmt_init()` 会像 Linux 一样注册 fallback `hci_mgmt_chan`，control bind 也要求 channel 已注册；sendmsg 经 channel handler table 分发，并执行 upstream 风格的 handler data length、`MGMT_INDEX_NONE` 和 controller index 校验。`READ_VERSION`、`READ_COMMANDS`、`READ_INDEX_LIST`、`READ_INFO` 已能返回 Linux mgmt binary command-complete 形状，其中全局命令默认使用 `MGMT_INDEX_NONE=0xffff`，controller 命令默认使用 index 0。设置类命令暂时转发到已有最小 `linux_bt_mgmt_dispatch()`，随后会把 `MGMT_EV_CMD_COMPLETE` skb 放入 socket receive queue，并可通过 staging `recvmsg()` 读回；成功设置还会额外排入 `MGMT_EV_NEW_SETTINGS`。`START_DISCOVERY` / `STOP_DISCOVERY` 现在维护一份 staging discovery state，并向其他 control socket 入队 `MGMT_EV_DISCOVERING`。它是 BlueZ mgmt control-channel 的语义探针，不表示完整 upstream `mgmt.c`/`hci_sock.c` 已经接管。
   - `btctl upstream mgmt-poll-discovery [max]` 会在 discovery active 时读取 `SIM_BTHWSIM` ADV raw record，并把 `HCI_LE_ADV_REPORT ... name=...` 转成 Linux 形状 `MGMT_EV_DEVICE_FOUND`。当前地址由 peer sim role 合成稳定 LE public address，EIR 使用 Complete Local Name；后续应由 upstream `hci_event.c` 的 LE Advertising Report 解析和 upstream `mgmt_device_found()` 接管。
   - `PAIR_DEVICE` / `CANCEL_PAIR_DEVICE` / `UNPAIR_DEVICE` 已补第一层 staging：`btctl upstream mgmt-socket 0x0019 0 <addr-seed>` 会发送 Linux `mgmt_cp_pair_device`，使用合成 LE public address 和 NoInputNoOutput IO capability，创建或更新 staging device-list entry 并立即标记 paired；`0x001a` 会走 `MGMT_OP_CANCEL_PAIR_DEVICE`，当前 immediate-complete 模式下通常无 pending pair 可取消；`0x001b` 会发送 `mgmt_cp_unpair_device`，默认 `disconnect=1`，清除 paired 状态，必要时清理 matching connection snapshot，并向其他 control socket 入队 `MGMT_EV_DEVICE_UNPAIRED`。后续应由 upstream pending command、SMP 和 key distribution 接管。
   - compat `struct proto` 已补 `owner/obj_size`，`sk_alloc()` 会按 `proto->obj_size` 分配更大的 socket 私有对象；这对后续 upstream `hci_sock.c` 的 `struct hci_pinfo`、L2CAP/ISO socket 私有结构是必要条件。
   - 已为 upstream `hci_sock.c` 补直接 include 缺口：导入 Linux `hci_mon.h`，新增 `linux/compat.h` 的 `compat_ptr()`，并把 `linux/utsname.h` 移入共享 `wireless/linux_compat`，提供 `release/machine`。
   - 已继续补 upstream `hci_sock.c` 的 runtime 兼容：`hci_dev` wrapper 现在有 `flags/dev_flags/promisc`、`hci_dev_get()` 和 flag helpers；socket compat 现在有 `proto_ops`、`msghdr/msg_iter`、`sk_write_queue`、`sock_queue_rcv_skb()`、`sock_orphan()`、`datagram_poll()` 和标准 `sock_no_*` helpers。
   - HCI monitor/logging staging 已补第一层：`HCI_CHANNEL_MONITOR` socket 会登记到本地 monitor list，control socket open/close、mgmt command 和 mgmt event 会按 `hci_mon_hdr` 形状广播为 `HCI_MON_CTRL_OPEN/CLOSE/COMMAND/EVENT`；`HCI_CHANNEL_LOGGING` sendmsg 会校验 logging frame，并按 `HCI_MON_USER_LOGGING` 转发给 monitor socket。这是后续接入 upstream `hci_sock.c` monitor replay 和 BlueZ `btmon` 可观测面的入口。
   - RAW/USER HCI socket TX staging 已补第一层：`HCI_CHANNEL_RAW` / `HCI_CHANNEL_USER` sendmsg 现在接受 Linux HCI socket 的 H4 形状，首字节为 HCI packet type，后续 payload 进入绑定 `hci_dev->send()`。发送前会检查已绑定 hdev、`HCI_UP` 和合法 packet type；通过 `btctl upstream mgmt-socket 0x0005 0 1` 设置 powered 时会同步 `hci_dev_open()`，使 RAW/USER TX 能按 Linux 语义从 user socket 到 upstream VHCI read queue。
   - RAW/USER HCI socket RX fanout 也已补第一层：`hci_recv_frame()` 在 controller/medium -> host 方向更新 HCI 统计后，会把匹配 `hci_dev` 的 RAW/USER socket 加入 receive queue，数据仍使用 Linux raw socket 可见的 H4 形状。RAW channel 接收 command/event/ACL/SCO/ISO，USER channel 接收 event/ACL/SCO/ISO/DRV；同时会向 monitor socket 广播 RX monitor event。`btctl upstream` 会输出 `hci-data-socket-register/unregister/rx` 计数。
   - `btctl upstream send cmd|acl|iso|event <payload>` 会构造 Linux `sk_buff`、设置 `hci_skb_pkt_type()`，再调用默认 `hci_dev->send()`，作为后续 upstream mgmt/hci_sock host->driver TX 的占位入口。
   - `btctl upstream sendhex cmd|acl|iso|event <hex...>` 和 `send` 使用同一条 TX 边界，但 payload 是真实字节，例如 HCI Reset command payload 为 `03 0c 00`。
   - `btaudio upstream-a2dp-source start` 会先走 staged `BTPROTO_L2CAP` socket `sendmsg()`，再把 synthetic A2DP media 打包成 HCI ACL + L2CAP CID `0x0041`，继续走 upstream VHCI `send -> readq -> drain -> hwsim ACL -> peer poll -> write_iter`。
   - `btaudio upstream-a2dp-sink start|read|stop` 会在接收端保持 staged L2CAP socket：`start` bind/listen A2DP media CID，`read` 先 poll hwsim 介质再从 socket receive queue 读取 payload，`stop` release socket。
   - `btaudio upstream-le-broadcast-source start [big] [bis]` 会先走 staged `BTPROTO_ISO` socket `sendmsg()`，再把 synthetic LE Audio media 打包成 HCI ISO，继续走 upstream VHCI ISO 数据面。
   - `btaudio upstream-le-broadcast-sink sync|start|stop [big] [bis] [max]` 会在接收端保持 staged ISO socket：`sync` bind/connect BIG/BIS handle，`start` poll ISO 介质并 recv ISO SDU，`stop` release socket。
   - `hwsim_bt1`、`hwsim_bt2`、`hwsim_ble1`、`hwsim_ble2` defconfig 现在显式启用 `NET_LINUX_BLUETOOTH_UPSTREAM_SOURCES`、`NET_LINUX_BLUETOOTH_UPSTREAM_LIB`、`NET_LINUX_BLUETOOTH_UPSTREAM_HCI_VHCI`、`NET_LINUX_BLUETOOTH_UPSTREAM_HCI_VHCI_AUTOSTART`、`DRIVERS_BLUETOOTH_LINUX` 和 `DRIVERS_BLUETOOTH_LINUX_HCI_VHCI`，并显式关闭 `NET_LINUX_BLUETOOTH_UPSTREAM_AF`，让四个 sim 入口默认带起 upstream `hci_vhci.c` 第一阶段和可运行的 staged socket/control/data 面。
   - 已为剩余 upstream host stack 文件补齐分阶段构建开关，其中 `AF_BLUETOOTH`、`HCI_CORE`、`HCI_SOCK`、`MGMT`、`L2CAP`、`SMP`、`ISO` 当前仍默认关闭。后续每打开一组，就以 Linux upstream 源文件替换一块临时 shim。
   - `AF_BLUETOOTH` 阶段已补第一层 socket compat：`sock_register()` / `sock_unregister()` staging registry、共享 `init_net`、`struct sock/socket/net_proto_family`、skb receive/error queue、wait/poll/ioctl/proc/seq 外壳，以及可调用的 `subsys_initcall(bt_init)` bridge。不过 Linux 原始 `af_bluetooth.c` 仍需要继续补完整 socket core、wait/ioctl/proc 和 `bt_sock` 依赖；当前 hwsim defconfig 暂不启用该源文件。
4. 当前临时 sim/hwsim shim：
   - `btctl` 触发 HCI control、LE ADV、ACL/L2CAP、ATT/GATT synthetic records。
   - `btaudio` 触发 A2DP ACL media 和 LE Audio ISO synthetic records。
   - connect/disconnect 已有 request/event/state 的第一阶段语义。
   - ACL poll 已能处理 `L2CAP_ECHO_REQ`、`ATT_READ_REQ`、`ATT_WRITE_REQ`、`AVDTP_START`，并生成对应 response record。
   - L2CAP signaling 已能处理 `CONN_REQ/RSP`、`CONFIG_REQ/RSP`、`DISCONN_REQ/RSP`，并推进 channel state。
   - SMP skeleton 已能处理 `PAIRING_REQ/RSP/CONFIRM/RANDOM` 的 Just Works 流程，并在 connection table 标记 `paired=1`。
   - 已新增本地对象表：connection table、L2CAP channel table、minimal ATT attribute table。
   - 已新增 ISO/BIG/BIS path table：记录 BIG、BIS、handle、source/sink、streaming/synced、codec 和 seq。
   - `btctl state` 可观察当前 role、connection、L2CAP channel、ATT DB 与 ISO path。
   - `btctl events` 可消费本地 HCI event ring，用于承载 command status/complete、connection complete、completed packets、BIG/BIS events 等本机 host 可见事件。
   - `btctl mgmt` 通过最小 Linux mgmt opcode dispatch 操作 controller settings：powered、connectable、discoverable、bondable、BR/EDR、LE、advertising。
5. 下一步在 NuttX 侧补齐更完整的 Linux-like HCI controller：
   - 优先适配 `upstream/net_bluetooth/{mgmt.c,hci_core.c,hci_event.c,hci_sock.c,l2cap_core.c,smp.c,iso.c}`。
  - 同步适配 `upstream/drivers_bluetooth/hci_vhci.c`，策略对齐 Wi-Fi `mac80211_hwsim_linux.c`：保持 Linux driver 源码接近原样，外侧只加 `drivers/bluetooth/linux_bt_upstream_vhci.c` 这样的 NuttX initialize/bind 层。
  - 当前 VHCI 阶段 wrapper 不能替代完整 Linux `hci_core.c`；它只是为了让 `hci_vhci.c` 的 driver 边界先和 NuttX sim/hwsim 介质对接。
   - 当前 host->driver TX 已有 `linux_bt_upstream_hci_send()` 入口；后续要由 upstream `mgmt.c` / `hci_sock.c` 生成真实 HCI command/ACL/ISO payload，而不是继续用调试字符串。
  - 当前 raw medium poll 已经进入 upstream `hci_vhci.c` 的 `write_iter()` / `hci_recv_frame()` 边界；后续要换成 upstream `hci_core.c` 的真实 rx work、event dispatch、L2CAP/ISO fanout。
   - 当前已有最小 command complete/status event ring；后续要替换成真正 HCI event queue 和 socket wakeup。
   - 当前已有最小 mgmt opcode dispatch facade；后续要替换成导入的 upstream `mgmt.c` socket opcode/event 分发。
   - staged build 顺序建议为：`AF_BLUETOOTH` -> `HCI_CORE` -> `HCI_SOCK/MGMT` -> `L2CAP` -> `SMP` -> `ISO`，每一步都以 upstream 源文件接管当前 shim；其中 `AF_BLUETOOTH` 已有兼容层雏形但当前默认关闭，`BTPROTO_HCI` 已有临时 create/bind/sendmsg 边界，`sk_alloc/proto_register`、`hci_sock.c` 直接 include 兼容和部分 HCI device/socket runtime 兼容已推进，下一步应继续拆 upstream `hci_sock.c` 的 mgmt channel、monitor replay、binary response/event queue 和 recvmsg 依赖。
   - 当前 `HCI_CHANNEL_CONTROL` 的 staging `sendmsg()` 已能接收 Linux mgmt header，并通过 fallback `hci_mgmt_chan` 支持 `READ_VERSION`、`READ_COMMANDS`、`READ_INDEX_LIST`、`READ_INFO`、`SET_POWERED`、`SET_DISCOVERABLE`、`SET_CONNECTABLE`、`SET_BONDABLE`、`SET_LE`、`SET_ADVERTISING`、`SET_BREDR`、`PAIR_DEVICE`、`CANCEL_PAIR_DEVICE`、`UNPAIR_DEVICE`、`START_DISCOVERY`、`STOP_DISCOVERY`、`BLOCK_DEVICE`、`UNBLOCK_DEVICE`、`GET_CONN_INFO`、`GET_CLOCK_INFO`、`ADD_DEVICE`、`REMOVE_DEVICE`、`GET_DEVICE_FLAGS`、`SET_DEVICE_FLAGS` 等 opcode。staging `recvmsg()` 现在会从 `sk_receive_queue` 取出 mgmt event skb，调用方缓冲不足时设置 `MSG_TRUNC`；`btctl upstream mgmt-socket` 使用较大的接收缓冲，能观察当前 `READ_INFO` / `READ_COMMANDS` binary response，并输出 recv flags。`btctl upstream` 状态会记录 `hci-mgmt-socket-cmd/event/recv`、`hci-mgmt-chan-register/unregister`、`hci-monitor-register/unregister/event` 和 `hci-data-socket-register/unregister/rx` 计数。这个边界服务于“一终端一个 sim”的 BlueZ-like 控制启动路径：每个 bt/ble sim 都能拥有自己的 upstream AF/VHCI/HCI control socket、RAW/USER HCI socket 和 monitor 可观测入口，再通过公共文件 ACL/ADV/ISO 介质和另一个 sim 通信。
   - ACL credit、完整 L2CAP socket/channel lifecycle、真实 SMP key distribution/security mode、完整 ATT/GATT database。
   - LE advertising report、connection complete、BIG/BIS/ISO events。
6. BT 基础闭环：
   - nsh 中用 Linux-port `btctl` 扫描、连接、发送基础 ACL payload。
7. BLE 基础闭环：
   - nsh 中用 Linux-port `btctl` 完成 advertise/scan/connect/GATT read-write。
8. Audio 第一阶段：
   - A2DP：先模拟 AVDTP signaling + L2CAP media channel，synthetic frame 通过 hwsim ACL record。
   - LE Audio：先模拟 BIG/BIS + ISO SDU，synthetic frame 通过 hwsim ISO record。

## 预期 nsh 使用方式

四个终端分别运行：

```bash
build/nuttx-sim-bt1
build/nuttx-sim-bt2
build/nuttx-sim-ble1
build/nuttx-sim-ble2
```

BT 基础命令目标：

```text
nsh> btctl info
nsh> btctl mgmt status
nsh> btctl mgmt power on
nsh> btctl mgmt connectable on
nsh> btctl mgmt discoverable on
nsh> btctl state
nsh> btctl events
nsh> btctl upstream
nsh> btctl upstream open
nsh> btctl upstream create
nsh> btctl upstream send acl synthetic_payload
nsh> btctl upstream sendhex cmd 03 0c 00
nsh> btctl upstream read
nsh> btctl upstream drain
nsh> btctl upstream poll
nsh> btctl upstream pump
nsh> btctl upstream socket hci
nsh> btctl upstream socket hci raw 0
nsh> btctl upstream socket hci control
nsh> btctl upstream mgmt-socket 0x0001
nsh> btctl upstream mgmt-socket 0x0002
nsh> btctl upstream mgmt-socket 0x0003
nsh> btctl upstream mgmt-socket 0x0004 0
nsh> btctl upstream mgmt-socket 0x0005 0 1
nsh> btctl upstream socket-send raw 0 cmd 03 0c 00
nsh> btctl upstream socket-filter 0 0xffffffff 0xffffffff 0xffffffff
nsh> btctl upstream socket-ioctl 0
nsh> btctl upstream socket-ioctl 0 up
nsh> btctl upstream socket-ioctl 0 down
nsh> btctl upstream socket-ioctl 0 reset
nsh> btctl upstream socket-ioctl 0 restat
nsh> btctl upstream socket-ioctl 0 scan 3
nsh> btctl upstream socket-ioctl 0 linkmode 0x8001
nsh> btctl upstream socket-ioctl 0 aclmtu 0x00400010
nsh> btctl upstream socket-ioctl 0 connlist
nsh> btctl upstream socket-ioctl 0 conninfo acl
nsh> btctl upstream socket-ioctl 0 conninfo bis
nsh> btctl upstream socket-ioctl 0 authinfo
nsh> btctl upstream socket-ioctl 0 block 1
nsh> btctl upstream socket-ioctl 0 unblock 1
nsh> btctl upstream mgmt-socket 0x0019 0 1
nsh> btctl upstream mgmt-socket 0x001a 0 1
nsh> btctl upstream mgmt-socket 0x001b 0 1
nsh> btctl upstream mgmt-socket 0x0023 0 7
nsh> btctl upstream mgmt-poll-discovery
nsh> btctl upstream mgmt-socket 0x0024 0 7
nsh> btctl upstream mgmt-socket 0x0026 0 1
nsh> btctl upstream mgmt-socket 0x0027 0 1
nsh> btctl upstream mgmt-socket 0x0031 0 0
nsh> btctl upstream mgmt-socket 0x0032 0 0
nsh> btctl upstream mgmt-socket 0x0033 0 1
nsh> btctl upstream mgmt-socket 0x004f 0 1
nsh> btctl upstream mgmt-socket 0x0050 0 1
nsh> btctl upstream mgmt-socket 0x0034 0 1
nsh> btctl upstream mgmt-listen
nsh> btctl upstream mgmt-read
nsh> btctl upstream mgmt-close
nsh> btctl upstream socket hci monitor
nsh> btctl upstream socket l2cap
nsh> btctl upstream socket iso
nsh> btctl upstream l2cap-bind 0x0019 0x0041 0x0040
nsh> btctl upstream l2cap-connect 0x0019 0x0041
nsh> btctl upstream l2cap-listen
nsh> btctl upstream l2cap-write 01 02 03 04
nsh> btctl upstream l2cap-send 0x0019 0x0041 0x0040 01 02 03 04
nsh> btctl upstream l2cap-recv
nsh> btctl upstream l2cap-close
nsh> btctl upstream iso-bind 0 0x0101
nsh> btctl upstream iso-connect 0
nsh> btctl upstream iso-write 01 02 03 04
nsh> btctl upstream iso-send 0 0x0101 01 02 03 04
nsh> btctl upstream iso-recv
nsh> btctl upstream iso-close
nsh> btctl scan bredr
nsh> btctl connect <peer>
nsh> btctl scan bredr
nsh> btctl pair <peer>
nsh> btctl l2cap-connect <peer> <psm>
nsh> btctl l2cap-disconnect <peer> <cid>
nsh> btctl l2cap-send <peer> <payload>
nsh> btctl l2cap-echo <peer> <payload>
nsh> btctl poll ctrl
nsh> btctl poll acl
```

两终端 L2CAP/A2DP-like staging 数据面示例：

```text
# 终端 A / 接收端，保持一个 L2CAP socket
nsh> btctl upstream l2cap-bind 0x0019 0x0041 0x0040
nsh> btctl upstream l2cap-connect 0x0019 0x0041

# 终端 B / 发送端，保持一个 L2CAP socket 并多次发送
nsh> btctl upstream l2cap-bind 0x0019 0x0041 0x0040
nsh> btctl upstream l2cap-connect 0x0019 0x0041
nsh> btctl upstream l2cap-write 01 02 03 04
nsh> btctl upstream pump

# 终端 A / 接收端，从公共文件介质 poll 回 host，再读 protocol socket queue
nsh> btctl upstream pump
nsh> btctl upstream l2cap-recv
nsh> btctl upstream l2cap-close
```

两终端 ISO/LE Audio-like staging 数据面示例：

```text
# 终端 A / 接收端，保持一个 ISO socket
nsh> btctl upstream iso-bind 0 0x0101
nsh> btctl upstream iso-connect 0

# 终端 B / 发送端，保持一个 ISO socket 并多次发送
nsh> btctl upstream iso-bind 0 0x0101
nsh> btctl upstream iso-connect 0
nsh> btctl upstream iso-write 01 02 03 04
nsh> btctl upstream pump

# 终端 A / 接收端，从公共文件介质 poll 回 host，再读 protocol socket queue
nsh> btctl upstream pump
nsh> btctl upstream iso-recv
nsh> btctl upstream iso-close
```

BLE 基础命令目标：

```text
nsh> btctl advertise start
nsh> btctl mgmt advertising on
nsh> btctl state
nsh> btctl scan le
nsh> btctl connect <peer>
nsh> btctl pair <peer>
nsh> btctl gatt-read [peer] <handle>
nsh> btctl gatt-write [peer] <handle> <payload>
nsh> btctl poll adv
nsh> btctl poll acl
```

Audio 命令目标：

```text
nsh> btaudio a2dp-source start [peer]
nsh> btaudio a2dp-sink start
nsh> btaudio upstream-a2dp-source start
nsh> btaudio upstream-a2dp-sink start|read|stop [max]
nsh> btaudio le-broadcast-source create|start|stop [big] [bis]
nsh> btaudio le-broadcast-sink sync|start|stop [big] [bis]
nsh> btaudio upstream-le-broadcast-source start [big] [bis]
nsh> btaudio upstream-le-broadcast-sink sync|start|stop [big] [bis] [max]
```

两终端 upstream 音频 socket 数据面示例：

```text
# A2DP-like receiver terminal
nsh> btaudio upstream-a2dp-sink start
nsh> btaudio upstream-a2dp-sink read
nsh> btaudio upstream-a2dp-sink stop

# A2DP-like sender terminal
nsh> btaudio upstream-a2dp-source start
nsh> btctl upstream pump

# LE Audio-like receiver terminal
nsh> btaudio upstream-le-broadcast-sink sync 0 1
nsh> btaudio upstream-le-broadcast-sink start 0 1
nsh> btaudio upstream-le-broadcast-sink stop

# LE Audio-like sender terminal
nsh> btaudio upstream-le-broadcast-source start 0 1
nsh> btctl upstream pump
```

这些 `btctl`/`btaudio` 命令已经作为 nsh 入口新增；后续需要把命令接入
Linux-port 栈核心。语义应对齐 Linux BlueZ 已验证过的 A2DP MediaTransport
与 LE Audio ISO socket 数据路径。

当前 synthetic record 行为：

- `btctl info` 调用 `linux_bt_info()` 输出当前 role/mode/capability。
- `btctl mgmt status` 调用 `linux_bt_mgmt_status()` 输出 Linux-like mgmt index、settings、supported settings。
- `btctl mgmt power|connectable|discoverable|bondable|le|bredr on|off` 调用对应 `linux_bt_mgmt_set_*()`，更新 controller settings，并向本地 event ring 写入 `MGMT_EV_NEW_SETTINGS`。
- `btctl mgmt advertising on|off` 通过 `linux_bt_advertise_start/stop()` 启停 LE advertising，同时更新 mgmt advertising setting。
- `btctl state` 调用 `linux_bt_state()` 输出本地 Linux-like connection table、L2CAP channel table 和 ATT DB。
- `btctl events` 调用 `linux_bt_events()` 输出并清空本地 HCI event ring；该 ring 不走公共文件介质，用于模拟 controller 上报给本机 host 的事件。
- `btctl advertise start` 调用 `linux_bt_advertise_start()`，写入 LE advertising semantic record。
- `btctl scan le` 调用 `linux_bt_scan_le()`，读取 peer/broadcast ADV record，并同时推进 LE control 面的 connect/disconnect state。
- `btctl scan bredr` 调用 `linux_bt_scan_bredr()`，读取 control record；如果读到发给本 role 的 `HCI_CMD_CONNECT`，会记录 connected state，并回写 `HCI_EVT_CONN_COMPLETE` 给发起端；如果读到 `HCI_CMD_DISCONNECT`，会清理 state，并回写 `HCI_EVT_DISCONN_COMPLETE`。
- `btctl connect <peer>` 调用 `linux_bt_connect()`，写入 `HCI_CMD_CONNECT` semantic record，并把本端 state 标记为 `connecting`，等待 peer 侧 scan/process 后再收到 complete event；收到 `HCI_EVT_CONN_COMPLETE` 后本端 state 切换为 `connected`。
- `btctl disconnect <peer>` 调用 `linux_bt_disconnect()`，写入 `HCI_CMD_DISCONNECT` semantic record，并清理本端 state。
- `btctl pair <peer>` 调用 `linux_bt_pair()`，通过 fixed SMP channel `cid=0x0006` 发起 Just Works pairing skeleton；对端 `btctl poll acl` 推进 `SMP_PAIRING_RSP/CONFIRM/RANDOM`，最终在 connection table 标记 `paired=1`。
- `btctl l2cap-connect <peer> <psm>` 调用 `linux_bt_l2cap_connect()`，通过 signaling CID `0x0001` 发起 `L2CAP_CONN_REQ`，对端 `btctl poll acl` 生成 `CONN_RSP` 和 `CONFIG_REQ`，本端再 poll 生成 `CONFIG_RSP`，最后 channel 进入 `open`。
- `btctl l2cap-disconnect <peer> <cid>` 调用 `linux_bt_l2cap_disconnect()`，通过 signaling CID `0x0001` 发送 `L2CAP_DISCONN_REQ`，对端 poll 后生成 `DISCONN_RSP` 并关闭 channel。
- `btctl l2cap-send <peer> <payload>` 调用 `linux_bt_l2cap_send()`，写入 `HCI_ACL_DATA` + L2CAP semantic record，并在本端打开 dynamic L2CAP channel `cid=0x0040 psm=0x1001`。
- `btctl l2cap-echo <peer> <payload>` 调用 `linux_bt_l2cap_echo()`，写入 signaling CID `L2CAP_ECHO_REQ`；对端 `btctl poll acl` 会生成 `L2CAP_ECHO_RSP`。
- `btctl gatt-read/write` 调用 `linux_bt_gatt_read/write()`，写入 broadcast ATT semantic record。
- `btctl gatt-read <peer> <handle>` 与 `btctl gatt-write <peer> <handle> <payload>` 会按 peer 生成 ACL handle，并在本端打开 fixed ATT channel `cid=0x0004`；对端 `btctl poll acl` 会从 minimal ATT DB 生成 `ATT_READ_RSP`、`ATT_WRITE_RSP` 或 `ATT_ERROR_RSP`。
- `btaudio a2dp-source start` 调用 `linux_bt_a2dp_source_start()`，写入 broadcast `HCI_ACL_DATA` + AVDTP/A2DP media synthetic frame，主要用于早期调试。
- `btaudio a2dp-source start <peer>` 调用 `linux_bt_a2dp_source_start_peer()`，按连接 peer 生成稳定 ACL handle，打开 AVDTP media channel `cid=0x0041 psm=0x0019`，并把 A2DP media synthetic frame 发给指定对端。
- `btaudio a2dp-sink start` 调用 `linux_bt_a2dp_sink_poll()`，读取 ACL media synthetic frame；如果收到 `AVDTP_START`，会生成 `AVDTP_START_RSP state=streaming`。
- `btaudio le-broadcast-source create [big] [bis]` 调用 `linux_bt_le_broadcast_source_create()`，创建 source ISO path，并生成 `HCI_EVT_LE_CREATE_BIG_COMPLETE`。
- `btaudio le-broadcast-source start [big] [bis]` 调用 `linux_bt_le_broadcast_source_start_path()`，确保 source ISO path 处于 `streaming`，递增 seq，并写入 `HCI_ISO_DATA` + BIG/BIS synthetic frame。
- `btaudio le-broadcast-source stop [big] [bis]` 调用 `linux_bt_le_broadcast_source_stop()`，清理 source ISO path，并生成 `HCI_EVT_LE_TERMINATE_BIG_COMPLETE`。
- `btaudio le-broadcast-sink sync [big] [bis]` 调用 `linux_bt_le_broadcast_sink_sync()`，创建 sink ISO path 并进入 `syncing`。
- `btaudio le-broadcast-sink start [big] [bis]` 调用 `linux_bt_le_broadcast_sink_poll_path()`，读取 ISO synthetic frame；收到记录后 sink ISO path 进入 `synced` 并累计 seq。
- `btaudio le-broadcast-sink stop [big] [bis]` 调用 `linux_bt_le_broadcast_sink_stop()`，清理 sink ISO path，并生成 `HCI_EVT_LE_BIG_SYNC_LOST`。

## 第一阶段双终端操作示例

BT1 连接 BT2：

```text
bt1 nsh> btctl mgmt power on
bt2 nsh> btctl mgmt power on
bt1 nsh> btctl mgmt connectable on
bt2 nsh> btctl mgmt connectable on
bt1 nsh> btctl connect 2
bt1 nsh> btctl events
bt2 nsh> btctl scan bredr
bt2 nsh> btctl events
bt1 nsh> btctl scan bredr
bt1 nsh> btctl events
```

L2CAP Echo：

```text
bt1 nsh> btctl l2cap-echo 2 hello
bt2 nsh> btctl poll acl
bt1 nsh> btctl poll acl
```

L2CAP dynamic channel：

```text
bt1 nsh> btctl l2cap-connect 2 0x1001
bt2 nsh> btctl poll acl
bt1 nsh> btctl poll acl
bt2 nsh> btctl poll acl
bt1 nsh> btctl state
bt2 nsh> btctl state
bt1 nsh> btctl l2cap-disconnect 2 0x0040
bt2 nsh> btctl poll acl
bt1 nsh> btctl poll acl
```

BLE advertise/connect/GATT：

```text
ble1 nsh> btctl mgmt power on
ble2 nsh> btctl mgmt power on
ble1 nsh> btctl mgmt le on
ble2 nsh> btctl mgmt le on
ble2 nsh> btctl mgmt advertising on
ble1 nsh> btctl scan le
ble1 nsh> btctl connect 4
ble2 nsh> btctl scan le
ble1 nsh> btctl scan le
ble1 nsh> btctl pair 4
ble2 nsh> btctl poll acl
ble1 nsh> btctl poll acl
ble2 nsh> btctl poll acl
ble1 nsh> btctl poll acl
ble1 nsh> btctl gatt-read 4 0x0001
ble1 nsh> btctl events
ble2 nsh> btctl poll acl
ble2 nsh> btctl events
ble1 nsh> btctl poll acl
ble1 nsh> btctl events
ble1 nsh> btctl state
ble2 nsh> btctl state
```

A2DP synthetic media：

```text
bt1 nsh> btaudio a2dp-source start 2
bt2 nsh> btaudio a2dp-sink start
bt1 nsh> btctl poll acl
```

LE Audio broadcast synthetic ISO：

```text
ble1 nsh> btaudio le-broadcast-source create 0 1
ble1 nsh> btctl state
ble2 nsh> btaudio le-broadcast-sink sync 0 1
ble2 nsh> btctl state
ble1 nsh> btaudio le-broadcast-source start 0 1
ble1 nsh> btctl events
ble2 nsh> btaudio le-broadcast-sink start 0 1
ble2 nsh> btctl events
ble2 nsh> btctl state
ble1 nsh> btaudio le-broadcast-source stop 0 1
ble2 nsh> btaudio le-broadcast-sink stop 0 1
```

## 2026-06-11 mgmt IO capability staging

本轮继续补齐 Linux mgmt control-channel 的配对前置语义：

```text
MGMT_OP_SET_IO_CAPABILITY 0x0018
  payload: struct mgmt_cp_set_io_capability { io_capability }
  response: MGMT_EV_CMD_COMPLETE + 原 payload
  state: 更新 staging controller 默认 IO capability
```

`btctl upstream mgmt-socket` 现在可以先设置 IO capability，再触发 synthetic pair：

```bash
btctl upstream mgmt-socket 0x0018 0 3
btctl upstream mgmt-socket 0x0019 0 1
```

其中 `0x0019` 的 `PAIR_DEVICE` payload 不再固定写死 NoInputNoOutput，而是使用最近一次 `SET_IO_CAPABILITY` 保存的 controller 默认值。这一步仍然是 staging immediate-complete pair，后续要继续由 upstream SMP、pending command 和 key distribution 接管。

## 2026-06-11 mgmt user confirmation staging

继续补齐 Linux pairing control path：当 `PAIR_DEVICE` 使用的 IO capability 不是 `NoInputNoOutput(0x03)` 时，staging device entry 不再立即标记 paired，而是进入 pending confirmation 状态，并向其他 control socket 广播：

```text
MGMT_EV_USER_CONFIRM_REQUEST 0x000f
```

随后可从 nsh 发送确认或否认：

```bash
btctl upstream mgmt-socket 0x0018 0 1
btctl upstream mgmt-socket 0x0019 0 1
btctl upstream mgmt-socket 0x001c 0 1

btctl upstream mgmt-socket 0x001d 0 1
```

`0x001c` 对应 `MGMT_OP_USER_CONFIRM_REPLY`，会把 pending entry 标记为 paired；`0x001d` 对应 `MGMT_OP_USER_CONFIRM_NEG_REPLY`，会清除 pending 状态并保持 unpaired。当前 command-complete 时机仍是 staging 简化模型，后续要继续替换为 upstream `mgmt_pending_add()`、SMP 和 HCI user confirmation command complete 语义。

## 2026-06-11 mgmt pending-command staging

本轮把 pairing staging 从单纯 device-state pending 推进到更接近 Linux `mgmt_pending_add()` 的形态：

```text
PAIR_DEVICE(non-NoInputNoOutput)
  -> 保存发起 command 的 control socket / opcode / index / addr
  -> 广播 MGMT_EV_USER_CONFIRM_REQUEST
  -> 不立即返回 PAIR_DEVICE command-complete

USER_CONFIRM_REPLY
  -> 标记 paired
  -> 给原 PAIR_DEVICE socket 补发 MGMT_EV_CMD_COMPLETE(success)

USER_CONFIRM_NEG_REPLY / CANCEL_PAIR_DEVICE / REMOVE_DEVICE
  -> 清理 pending
  -> 给原 PAIR_DEVICE socket 补发失败或取消形态的 command-complete
```

这比前一版更接近 upstream `net/bluetooth/mgmt.c` 的 pending command ownership，但仍然没有接入真正的 SMP pairing method、HCI user-confirm command、key distribution 和持久 key store。下一步应把 pending entry 与 imported `smp.c` 的 method/key flow 对接。

## 2026-06-11 persistent mgmt-send staging

新增常驻 control socket 发送入口：

```bash
btctl upstream mgmt-listen
btctl upstream mgmt-send 0x0018 0 1
btctl upstream mgmt-read
btctl upstream mgmt-send 0x0019 0 1
btctl upstream mgmt-read
btctl upstream mgmt-send 0x001c 0 1
btctl upstream mgmt-read
btctl upstream mgmt-close
```

`mgmt-send` 和 `mgmt-socket` 的区别是：`mgmt-send` 复用 `mgmt-listen` 打开的同一个 HCI control socket。这样 pending `PAIR_DEVICE` 保存的 socket owner 就能在后续 `USER_CONFIRM_REPLY` 后收到原 pair command-complete，更接近 BlueZ daemon 的常驻 mgmt socket 使用方式。

## 2026-06-11 mgmt passkey reply staging

新增 staged passkey pairing branch。当前 IO capability 约定：

```text
0x02: KeyboardOnly -> MGMT_EV_USER_PASSKEY_REQUEST
0x03: NoInputNoOutput -> immediate staged paired
其它非 0x03: MGMT_EV_USER_CONFIRM_REQUEST
```

可用 persistent mgmt socket 观察 passkey 路径：

```bash
btctl upstream mgmt-listen
btctl upstream mgmt-send 0x0018 0 2
btctl upstream mgmt-read
btctl upstream mgmt-send 0x0019 0 1
btctl upstream mgmt-read
btctl upstream mgmt-send 0x001e 0 1
btctl upstream mgmt-read
btctl upstream mgmt-close
```

`0x001e` 对应 `MGMT_OP_USER_PASSKEY_REPLY`，当前 staged passkey 固定为 `123456`；`0x001f` 对应 `MGMT_OP_USER_PASSKEY_NEG_REPLY`，会失败并清除 pending pair。后续要把 passkey request/reply 与 upstream `smp.c` 的 method selection、confirm/random/key distribution 连接起来。

## 2026-06-11 staged LE LTK distribution

pairing 成功路径现在会生成一条 staged LE Long Term Key，并通过 mgmt control socket 广播：

```text
MGMT_EV_NEW_LONG_TERM_KEY 0x000a
  store_hint = 1
  key.addr = peer LE address
  key.type = unauthenticated 或 authenticated
  key.enc_size = 16
```

触发路径包括：

```text
NoInputNoOutput immediate pair success
USER_CONFIRM_REPLY success
USER_PASSKEY_REPLY success
```

这一步让 BlueZ/mgmt 可观察到 bonding key distribution 的第一块形状。当前 LTK 值仍由 staging 按 peer address 确定性生成，不代表真实 SMP confirm/random 派生结果；IRK、CSRK、BR/EDR link key 和持久 key store 还未接入。

## 2026-06-11 BT/BLE hwsim usecase test matrix

新增用例生成入口：

```bash
tools/firmware/sim/test-bt-hwsim-usecases.sh list
tools/firmware/sim/test-bt-hwsim-usecases.sh write
tools/firmware/sim/test-bt-hwsim-usecases.sh show bt-basic
```

默认会把每个用例的 per-terminal nsh 命令文件写到：

```text
build/bt-hwsim-usecases
```

当前覆盖矩阵：

```text
bt-basic:     bt1/bt2 BR/EDR mgmt、scan/connect/pair、L2CAP echo、state/events
ble-basic:    ble1/ble2 LE advertising/scan/connect/pair、GATT read/write
mgmt-noio:    persistent mgmt socket、NoInputNoOutput pair、LTK event
mgmt-confirm: persistent mgmt socket、user-confirm pair、pending complete
mgmt-passkey: persistent mgmt socket、passkey pair、pending complete
a2dp:         upstream L2CAP socket A2DP-like payload path
le-audio:     upstream ISO socket LE Audio-like payload path
```

运行模型仍保持和目标一致：一个 sim 占一个终端。脚本只生成命令文件，不自动启动四个 sim，也不替代真实日志判定。后续可以在这个矩阵上继续叠加 tmux/expect 自动化和 PASS/FAIL 日志解析。

## 2026-06-11 BT/BLE hwsim log validator

新增用例日志校验器：

```bash
tools/firmware/sim/validate-bt-hwsim-usecases.py --list
tools/firmware/sim/validate-bt-hwsim-usecases.py \
  --log-dir build/bt-hwsim-usecases
tools/firmware/sim/validate-bt-hwsim-usecases.py \
  --log-dir build/bt-hwsim-usecases --case mgmt-passkey --json
```

日志命名约定：

```text
<case>.<role>.log
```

例如：

```text
bt-basic.bt1.log
bt-basic.bt2.log
mgmt-passkey.ble1.log
le-audio.ble2.log
```

当前 validator 检查每个用例的关键用户可见输出：mgmt command/event、pair pending complete、LTK event、L2CAP/A2DP payload path 和 ISO/LE Audio payload path。它还会把 `PANIC`、`ASSERT`、`btctl: ... failed:`、`btaudio: ... failed:` 视为失败。真实四终端运行后，应把终端输出保存为上述日志，再运行 validator 作为 PASS/FAIL 记录入口。

## 2026-06-11 BT/BLE hwsim usecase runner

新增真实 sim 用例 runner：

```bash
tools/firmware/sim/run-bt-hwsim-usecases.py --case mgmt-passkey
tools/firmware/sim/run-bt-hwsim-usecases.py --case bt-basic --case a2dp
tools/firmware/sim/run-bt-hwsim-usecases.py
```

runner 会：

```text
1. 调用 test-bt-hwsim-usecases.sh write 生成 nsh 命令文件。
2. 按用例启动需要的 build/nuttx-sim-<role> 进程。
3. 向每个 role 的 stdin 写入对应 <case>.<role>.nsh 命令。
4. 保存 <case>.<role>.log。
5. 调用 validate-bt-hwsim-usecases.py 输出 PASS/FAIL。
```

默认日志目录：

```text
build/bt-hwsim-usecases
```

注意：这个 runner 是自动化入口，会真实启动 sim 进程；运行前应先完成对应构建入口：

```bash
tools/firmware/sim/build-bt1.sh
tools/firmware/sim/build-bt2.sh
tools/firmware/sim/build-ble1.sh
tools/firmware/sim/build-ble2.sh
```

本轮只新增 runner，没有实际启动四个 sim，也没有记录真实 PASS/FAIL。

## 2026-06-11 BT/BLE hwsim runner result manifest

`run-bt-hwsim-usecases.py` 现在会在日志目录写入机器可读结果清单：

```text
build/bt-hwsim-usecases/run-results.json
```

清单字段包括：

```text
cases: 本次运行的 case 列表
results: 每个 case 的 role、日志路径和 run_error
validate_rc: validate-bt-hwsim-usecases.py 返回码
passed: runner 和 validator 都成功时为 true
```

后续真实测试记录应同时保留：

```text
<case>.<role>.log
run-results.json
validator 文本输出或 --json 输出
```

这让“测试各个用例”的结果可以作为可审计证据，而不是只依赖终端肉眼观察。

## 2026-06-11 BT/BLE hwsim test preflight result

本轮执行了轻量 preflight 检查，结果：

```text
MISSING build/nuttx-sim-bt1
MISSING build/nuttx-sim-bt2
MISSING build/nuttx-sim-ble1
MISSING build/nuttx-sim-ble2
python syntax: PASS
usecase generator list: PASS
validator list: PASS
```

因此当前不能直接启动真实 usecase runner；需要先构建四个角色。新增 preflight 入口：

```bash
tools/firmware/sim/preflight-bt-hwsim-usecases.sh
```

构建入口仍是：

```bash
tools/firmware/sim/build-bt1.sh
tools/firmware/sim/build-bt2.sh
tools/firmware/sim/build-ble1.sh
tools/firmware/sim/build-ble2.sh
```
## 2026-06-14 BT/BLE NET upstream convergence closeout

新增 `bluez-net-upstream-convergence-closeout`。

验证结果：PASS

```text
case:     bluez-net-upstream-convergence-closeout
roles:    bt1, bt2, ble1, ble2
run:      build/logs/run-bluez-net-upstream-convergence-closeout.log
manifest: build/bt-hwsim-usecases-bluez-net-upstream-convergence-closeout/run-results.json
```

该 gate 强制检查 BT Network/BNEP 和 BLE IPSP/6LoWPAN 的 `upstream-coverage-map`，覆盖 BlueZ Network/IPSP 源文件、Linux BNEP/6LoWPAN/L2CAP/IPHC 源文件，以及 `btn0` / `bt0` 数据面和 final cleanup。

边界仍是 staged adapter：BT `blueznetwork-adapter-not-unmodified-bluetoothd`，BLE `bluezdaemon-ipsp-adapter-not-unmodified-bluetoothd`。

## 2026-06-14 A2DP upstream convergence closeout

新增 `bluez-a2dp-upstream-convergence-closeout`。

验证结果：PASS

```text
case:     bluez-a2dp-upstream-convergence-closeout
roles:    bt1, bt2
run:      build/logs/run-bluez-a2dp-upstream-convergence-closeout.log
manifest: build/bt-hwsim-usecases-bluez-a2dp-upstream-convergence-closeout/run-results.json
```

该 gate 强制检查 `upstream-coverage-map`，覆盖 BlueZ `profile/device/adapter/dbus/mainloop/sdpd/audio` 源文件、Linux L2CAP 源文件，以及 source/sink 双端 profile/dbus/mainloop/transaction/media/codec/transport/AVRCP/L2CAP/state/cleanup final evidence。

边界仍是 staged adapter：`bluezdaemon-adapter-not-unmodified-bluetoothd`。

## 2026-06-14 BT/BLE NET current complete closeout

新增 `bluez-net-current-complete-closeout`，用于把 BT Network/BNEP 和 BLE IPSP/6LoWPAN 的基础数据面收成一个四角色 hwsim gate。

验证结果：PASS

```text
case:     bluez-net-current-complete-closeout
roles:    bt1, bt2, ble1, ble2
run:      build/logs/run-bluez-net-current-complete-closeout.log
manifest: build/bt-hwsim-usecases-bluez-net-current-complete-closeout/run-results.json
```

覆盖 BT `btn0` ping reply、BNEP native TX/L2CAP/RX/netif counters、PANU/NAP/GN role lifecycle、error cleanup，以及 BLE `bt0` ping6、TCP/UDP iperf、IPHC/fragment counters、Profile1/mainloop ownership 和 final cleanup。

边界：该综合 gate 负责基础能力收口；BT Network MTU1400 压测仍由 `bluez-network-closeout-full` 单项 gate 覆盖。

## 2026-06-14 A2DP upstream AVDTP signaling sequence closeout

`bluez-a2dp-upstream-convergence-closeout` was rerun after adding an AVDTP signaling sequence gate to the upstream handler bridge surface.

- Result: PASS
- Run log: `build/logs/run-a2dp-avdtp-signaling-closeout.log`
- Manifest: `build/bt-hwsim-usecases-a2dp-avdtp-signaling-closeout/run-results.json`
- New required marker: `upstream-avdtp-signaling=discover:1,getcap:1,set-config:1,open:1,start:1,suspend:1,close:1,abort:1,total:8`

This keeps the staged adapter boundary explicit while moving the A2DP closeout closer to upstream BlueZ AVDTP request/response ordering and lifecycle semantics.

## 2026-06-14 A2DP upstream linked handler/mainloop closeout

`bluez-a2dp-upstream-convergence-closeout` was rerun after adding the linked handler/mainloop gate.

- Result: PASS
- Run log: `build/logs/run-a2dp-linked-handler-mainloop.log`
- Manifest: `build/bt-hwsim-usecases-a2dp-linked-handler-mainloop/run-results.json`
- New required marker: `upstream-linked-handler-mainloop=transport-dispatch:1,media-dispatch:1,pending:1,watch:1,cleanup:1,total:5`

This gate checks the shared dispatch path plus pending request, mainloop watch, fd handoff, reply/error, and cleanup lifecycle semantics for the staged upstream handler bridge.

## 2026-06-14 A2DP upstream profile daemon flow closeout

`bluez-a2dp-upstream-convergence-closeout` was rerun after adding the profile-daemon lifecycle gate.

- Result: PASS
- Run log: `build/logs/run-a2dp-profile-daemon-flow.log`
- Manifest: `build/bt-hwsim-usecases-a2dp-profile-daemon-flow/run-results.json`
- New required marker: `upstream-a2dp-profile-daemon=plugin-init:1,adapter-probe:1,endpoint-register:1,avdtp-bind:1,transport-export:1,daemon-cleanup:1,total:6`

This gate checks plugin/init, adapter probe, endpoint registration, AVDTP binding, transport export, and daemon cleanup semantics in the staged upstream bridge.

## 2026-06-14 A2DP upstream audio header/API link probe closeout

`bluez-a2dp-upstream-convergence-closeout` was rerun after adding `bluez/upstream_audio_link_probe.c` to the `bluezdaemon` build.

- Result: PASS
- Run log: `build/logs/run-a2dp-upstream-audio-link-probe.log`
- Manifest: `build/bt-hwsim-usecases-a2dp-upstream-audio-link-probe/run-results.json`
- New required marker: `upstream-audio-link-probe ... api=headers:1,callbacks:1,constants:1,profile:1,transport:1,total:5 ... final-ok=1`

This is a build-chain step toward real upstream BlueZ linkage: upstream audio headers are now compiled from the symlinked upstream tree by an apps-side object, while upstream audio implementation `.c` objects remain a future step.

## 2026-06-14 A2DP upstream AVDTP packet implementation probe closeout

`bluez-a2dp-upstream-convergence-closeout` was rerun after adding the AVDTP packet implementation probe.

- Result: PASS
- Run log: `build/logs/run-a2dp-avdtp-packet-impl-probe.log`
- Manifest: `build/bt-hwsim-usecases-a2dp-avdtp-packet-impl-probe/run-results.json`
- New required marker: `upstream-avdtp-packet-impl-probe ... impl=packet-headers:1,transactions:1,fragments:1,signals:1,total:4 ... final-ok=1`

This validates a ported upstream AVDTP packet/header implementation slice while keeping the boundary that the full upstream `avdtp.c` object is not yet linked unmodified.

## 2026-06-14 A2DP upstream AVDTP parse implementation probe closeout

`bluez-a2dp-upstream-convergence-closeout` was rerun after adding the AVDTP parse implementation probe.

- Result: PASS
- Run log: `build/logs/run-a2dp-avdtp-parse-impl-probe.log`
- Manifest: `build/bt-hwsim-usecases-a2dp-avdtp-parse-impl-probe/run-results.json`
- New required marker: `upstream-avdtp-parse-impl-probe ... impl=single:1,start:1,continue:1,end:1,transaction-mismatch:1,route:1,total:6 ... final-ok=1`

This validates a ported upstream AVDTP parse-path implementation slice while keeping the boundary that the full upstream `avdtp.c` object is not yet linked unmodified.

## 2026-06-14 A2DP upstream AVDTP signal implementation probe closeout

`bluez-a2dp-upstream-convergence-closeout` was rerun after adding the AVDTP signal implementation probe.

- Result: PASS
- Run log: `build/logs/run-a2dp-avdtp-signal-impl-probe.log`
- Manifest: `build/bt-hwsim-usecases-a2dp-avdtp-signal-impl-probe/run-results.json`
- New required marker: `upstream-avdtp-signal-impl-probe ... impl=command-dispatch:1,accept-response:1,reject-response:1,error-map:1,total:4 ... final-ok=1`

This validates a ported upstream AVDTP signal-path implementation slice while keeping the boundary that the full upstream `avdtp.c` object is not yet linked unmodified.

## 2026-06-14 A2DP upstream AVDTP stream implementation probe closeout

`bluez-a2dp-upstream-convergence-closeout` was rerun after adding the AVDTP stream implementation probe.

- Result: PASS
- Run log: `build/logs/run-a2dp-avdtp-stream-impl-probe.log`
- Manifest: `build/bt-hwsim-usecases-a2dp-avdtp-stream-impl-probe/run-results.json`
- New required marker: `upstream-avdtp-stream-impl-probe ... impl=state-machine:1,timers:1,pending-open:1,callbacks:1,cleanup:1,total:5 ... final-ok=1`

This validates a ported upstream AVDTP stream lifecycle implementation slice while keeping the boundary that the full upstream `avdtp.c` object is not yet linked unmodified.

## 2026-06-14 A2DP upstream setup implementation probe closeout

`bluez-a2dp-upstream-convergence-closeout` was rerun after adding the A2DP setup implementation probe.

- Result: PASS
- Run log: `build/logs/run-a2dp-setup-impl-probe.log`
- Manifest: `build/bt-hwsim-usecases-a2dp-setup-impl-probe/run-results.json`
- New required marker: `upstream-a2dp-setup-impl-probe ... impl=setup-refs:1,callbacks:1,sep-lock:1,stream-attach:1,error-cleanup:1,total:5 ... final-ok=1`

This validates a ported upstream A2DP setup/SEP/stream implementation slice while keeping the boundary that the full upstream `a2dp.c` object is not yet linked unmodified.

## 2026-06-14 A2DP MediaTransport closeout marker

`bluez-a2dp-upstream-convergence-closeout` now validates this additional marker on both roles:

```text
bluez-daemon: a2dp upstream-media-transport-impl-probe role=<source|sink> compile-unit=bluez/upstream_audio_link_probe.c source=third/bluez/profiles/audio/transport.c impl=state:1,owner:1,acquire-release:1,properties:1,cleanup:1,total:5 boundary=upstream-media-transport-impl-ported-not-yet-transport-c-object final-ok=1
```

Latest evidence:

- `build/logs/build-bt1-a2dp-media-transport-impl-probe.log`
- `build/logs/build-bt2-a2dp-media-transport-impl-probe.log`
- `build/logs/run-a2dp-media-transport-impl-probe.log`
- `build/bt-hwsim-usecases-a2dp-media-transport-impl-probe/run-results.json`

Result: `PASS bluez-a2dp-upstream-convergence-closeout`.

## 2026-06-14 A2DP transport D-Bus FSM gate

The `bluez-a2dp-upstream-convergence-closeout` case now requires the following source and sink marker:

```text
upstream-transport-dbus-fsm=acquire:1,try-acquire:1,release:1,select-unselect:1,error:1,final-zero:1,total:6
```

Latest evidence:

- `build/logs/build-bt1-a2dp-transport-dbus-fsm.log`
- `build/logs/build-bt2-a2dp-transport-dbus-fsm.log`
- `build/logs/run-a2dp-transport-dbus-fsm.log`
- `build/bt-hwsim-usecases-a2dp-transport-dbus-fsm/run-results.json`

Result: `PASS bluez-a2dp-upstream-convergence-closeout`.

## 2026-06-14 A2DP media endpoint D-Bus FSM gate

The `bluez-a2dp-upstream-convergence-closeout` case now requires the following source and sink marker:

```text
upstream-media-endpoint-dbus-fsm=register:1,select:1,set:1,clear:1,unregister:1,error:1,final-zero:1,total:7
```

Latest evidence:

- `build/logs/build-bt1-a2dp-media-endpoint-dbus-fsm.log`
- `build/logs/build-bt2-a2dp-media-endpoint-dbus-fsm.log`
- `build/logs/run-a2dp-media-endpoint-dbus-fsm.log`
- `build/bt-hwsim-usecases-a2dp-media-endpoint-dbus-fsm/run-results.json`

Result: `PASS bluez-a2dp-upstream-convergence-closeout`.

## 2026-06-14 A2DP media application D-Bus FSM gate

The `bluez-a2dp-upstream-convergence-closeout` case now requires the following source and sink marker:

```text
upstream-media-application-dbus-fsm=register:1,endpoints:1,players:1,unregister:1,disconnect:1,error:1,final-zero:1,total:7
```

Latest evidence:

- `build/logs/build-bt1-a2dp-media-application-dbus-fsm.log`
- `build/logs/build-bt2-a2dp-media-application-dbus-fsm.log`
- `build/logs/run-a2dp-media-application-dbus-fsm.log`
- `build/bt-hwsim-usecases-a2dp-media-application-dbus-fsm/run-results.json`

Result: `PASS bluez-a2dp-upstream-convergence-closeout`.

## 2026-06-14 A2DP AVRCP profile FSM gate

The `bluez-a2dp-upstream-convergence-closeout` case now requires the following source and sink marker:

```text
upstream-avrcp-profile-fsm=player-register:1,controller:1,target:1,metadata:1,volume:1,disconnect:1,final-zero:1,total:7
```

Latest evidence:

- `build/logs/build-bt1-a2dp-avrcp-profile-fsm.log`
- `build/logs/build-bt2-a2dp-avrcp-profile-fsm.log`
- `build/logs/run-a2dp-avrcp-profile-fsm.log`
- `build/bt-hwsim-usecases-a2dp-avrcp-profile-fsm/run-results.json`

Result: `PASS bluez-a2dp-upstream-convergence-closeout`.

## 2026-06-14 A2DP media stream FSM gate

The `bluez-a2dp-upstream-convergence-closeout` case now requires the following source and sink marker:

```text
upstream-a2dp-media-stream-fsm=open:1,start:1,rtp:1,payload:1,suspend:1,close:1,error:1,final-zero:1,total:8
```

Latest evidence:

- `build/logs/build-bt1-a2dp-media-stream-fsm.log`
- `build/logs/build-bt2-a2dp-media-stream-fsm.log`
- `build/logs/run-a2dp-media-stream-fsm.log`
- `build/bt-hwsim-usecases-a2dp-media-stream-fsm/run-results.json`

Result: `PASS bluez-a2dp-upstream-convergence-closeout`.

## 2026-06-14 A2DP codec policy FSM gate

The `bluez-a2dp-upstream-convergence-closeout` case now requires the following source and sink marker:

```text
upstream-a2dp-codec-policy-fsm=capability:1,select:1,set:1,reconfigure:1,delay:1,error:1,final-zero:1,total:7
```

Latest evidence:

- `build/logs/build-bt1-a2dp-codec-policy-fsm.log`
- `build/logs/build-bt2-a2dp-codec-policy-fsm.log`
- `build/logs/run-a2dp-codec-policy-fsm.log`
- `build/bt-hwsim-usecases-a2dp-codec-policy-fsm/run-results.json`

Result: `PASS bluez-a2dp-upstream-convergence-closeout`.

## 2026-06-14 A2DP lifecycle stress FSM gate

The `bluez-a2dp-upstream-convergence-closeout` case now requires the following source and sink marker:

```text
upstream-a2dp-lifecycle-stress-fsm=first-connect:1,cleanup:1,reconnect:1,duplicate-reject:1,media-resume:1,disconnect:1,final-zero:1,total:7
```

Latest evidence:

- `build/logs/build-bt1-a2dp-lifecycle-stress-fsm.log`
- `build/logs/build-bt2-a2dp-lifecycle-stress-fsm.log`
- `build/logs/run-a2dp-lifecycle-stress-fsm.log`
- `build/bt-hwsim-usecases-a2dp-lifecycle-stress-fsm/run-results.json`

Result: `PASS bluez-a2dp-upstream-convergence-closeout`.

## 2026-06-14 A2DP upstream object-link readiness gate

The `bluez-a2dp-upstream-convergence-closeout` case now requires the following source and sink marker:

```text
upstream-a2dp-object-link-readiness=sources:1,headers:1,glib-dbus:1,mainloop:1,core-objects:1,l2cap-media:1,symbol-ownership:1,replacement-boundary:1,total:8
```

Latest evidence:

- `build/logs/build-bt1-a2dp-object-link-readiness.log`
- `build/logs/build-bt2-a2dp-object-link-readiness.log`
- `build/logs/run-a2dp-object-link-readiness.log`
- `build/bt-hwsim-usecases-a2dp-object-link-readiness/run-results.json`

Result: `PASS bluez-a2dp-upstream-convergence-closeout`.

## 2026-06-14 A2DP negative/boundary FSM gate

The `bluez-a2dp-upstream-convergence-closeout` case now requires the following source and sink marker:

```text
upstream-a2dp-negative-boundary-fsm=bad-state:1,mtu:1,fd:1,codec-recover:1,duplicate-request:1,abort-cleanup:1,final-zero:1,total:7
```

Latest evidence:

- `build/logs/build-bt1-a2dp-negative-boundary-fsm.log`
- `build/logs/build-bt2-a2dp-negative-boundary-fsm.log`
- `build/logs/run-a2dp-negative-boundary-fsm.log`
- `build/bt-hwsim-usecases-a2dp-negative-boundary-fsm/run-results.json`

Result: `PASS bluez-a2dp-upstream-convergence-closeout`.
