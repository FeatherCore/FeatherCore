/****************************************************************************
 * tools/firmware/sim/x11-input.c
 *
 * SPDX-License-Identifier: Apache-2.0
 *
 ****************************************************************************/

#define _DEFAULT_SOURCE

/****************************************************************************
 * Included Files
 ****************************************************************************/

#include <X11/Xatom.h>
#include <X11/Xlib.h>
#include <X11/Xutil.h>
#include <X11/keysym.h>

#ifdef HAVE_XTEST
#  include <X11/extensions/XTest.h>
#endif

#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <unistd.h>

/****************************************************************************
 * Private Data
 ****************************************************************************/

static int g_x11_input_trapped_error;

/****************************************************************************
 * Private Functions
 ****************************************************************************/

static int x11_input_trap_error(Display *display, XErrorEvent *event)
{
  (void)display;

  g_x11_input_trapped_error = event == NULL ? 0 : event->error_code;
  return 0;
}

static void x11_input_try_focus(Display *display, Window window)
{
  int (*previous)(Display *, XErrorEvent *);

  g_x11_input_trapped_error = 0;
  previous = XSetErrorHandler(x11_input_trap_error);

  XRaiseWindow(display, window);
  XSetInputFocus(display, window, RevertToParent, CurrentTime);
  XSync(display, False);

  XSetErrorHandler(previous);
  (void)g_x11_input_trapped_error;
}

static bool x11_input_name_matches(Display *display, Window window,
                                   const char *needle)
{
  XTextProperty property;
  char **list;
  char *name;
  int count;
  int i;
  bool matched;

  matched = false;
  name = NULL;
  if (XFetchName(display, window, &name) != 0 && name != NULL)
    {
      matched = strstr(name, needle) != NULL;
      XFree(name);
      if (matched)
        {
          return true;
        }
    }

  memset(&property, 0, sizeof(property));
  if (XGetWMName(display, window, &property) == 0 || property.value == NULL)
    {
      return false;
    }

  list = NULL;
  count = 0;
  if (XmbTextPropertyToTextList(display, &property, &list, &count) >=
      Success)
    {
      for (i = 0; i < count; i++)
        {
          if (list[i] != NULL && strstr(list[i], needle) != NULL)
            {
              matched = true;
              break;
            }
        }

      if (list != NULL)
        {
          XFreeStringList(list);
        }
    }

  XFree(property.value);
  return matched;
}

static Window x11_input_find_window(Display *display, Window root,
                                    const char *needle)
{
  Window parent;
  Window found;
  Window root_return;
  Window *children;
  unsigned int nchildren;
  unsigned int i;

  children = NULL;
  nchildren = 0;
  if (XQueryTree(display, root, &root_return, &parent, &children,
                 &nchildren) == 0)
    {
      return x11_input_name_matches(display, root, needle) ? root : None;
    }

  found = None;
  for (i = 0; i < nchildren; i++)
    {
      found = x11_input_find_window(display, children[i], needle);
      if (found != None)
        {
          break;
        }
    }

  if (children != NULL)
    {
      XFree(children);
    }

  if (found == None && x11_input_name_matches(display, root, needle))
    {
      found = root;
    }

  return found;
}

static bool x11_input_translate(Display *display, Window window,
                                int x, int y, int *root_x, int *root_y)
{
  Window child;

  *root_x = x;
  *root_y = y;
  return XTranslateCoordinates(display, window,
                               DefaultRootWindow(display),
                               x, y, root_x, root_y, &child) != 0;
}

static unsigned int x11_input_button_state_mask(unsigned int button)
{
  switch (button)
    {
      case 1:
        return Button1Mask;

      case 2:
        return Button2Mask;

      case 3:
        return Button3Mask;

      case 4:
        return Button4Mask;

      case 5:
        return Button5Mask;

      default:
        return 0;
    }
}

static int x11_input_send_motion_state(Display *display, Window window,
                                       int x, int y,
                                       unsigned int state)
{
  XEvent event;
  int root_x;
  int root_y;

  x11_input_try_focus(display, window);
  XWarpPointer(display, None, window, 0, 0, 0, 0, x, y);
  XFlush(display);
  XSync(display, False);

  (void)x11_input_translate(display, window, x, y, &root_x, &root_y);
  memset(&event, 0, sizeof(event));
  event.xmotion.type = MotionNotify;
  event.xmotion.display = display;
  event.xmotion.window = window;
  event.xmotion.root = DefaultRootWindow(display);
  event.xmotion.subwindow = None;
  event.xmotion.time = CurrentTime;
  event.xmotion.x = x;
  event.xmotion.y = y;
  event.xmotion.x_root = root_x;
  event.xmotion.y_root = root_y;
  event.xmotion.state = state;
  event.xmotion.same_screen = True;

  if (XSendEvent(display, window, True, PointerMotionMask, &event) == 0)
    {
      return 1;
    }

  XFlush(display);
  return 0;
}

static int x11_input_send_motion(Display *display, Window window,
                                 int x, int y)
{
  return x11_input_send_motion_state(display, window, x, y, 0);
}

static int x11_input_send_button(Display *display, Window window,
                                 int type, int x, int y, unsigned int button)
{
  XEvent event;
  int root_x;
  int root_y;
  long mask;

  x11_input_try_focus(display, window);
  XWarpPointer(display, None, window, 0, 0, 0, 0, x, y);
  XSync(display, False);

#ifdef HAVE_XTEST
  if (XTestFakeButtonEvent(display, button, type == ButtonPress,
                           CurrentTime) != 0)
    {
      XFlush(display);
      return 0;
    }
#endif

  (void)x11_input_translate(display, window, x, y, &root_x, &root_y);
  memset(&event, 0, sizeof(event));
  event.xbutton.type = type;
  event.xbutton.display = display;
  event.xbutton.window = window;
  event.xbutton.root = DefaultRootWindow(display);
  event.xbutton.subwindow = None;
  event.xbutton.time = CurrentTime;
  event.xbutton.x = x;
  event.xbutton.y = y;
  event.xbutton.x_root = root_x;
  event.xbutton.y_root = root_y;
  event.xbutton.button = button;
  event.xbutton.same_screen = True;
  event.xbutton.state = type == ButtonRelease ? Button1Mask : 0;

  mask = type == ButtonPress ? ButtonPressMask : ButtonReleaseMask;
  if (XSendEvent(display, window, True, mask, &event) == 0)
    {
      return 1;
    }

  XFlush(display);
  return 0;
}

static int x11_input_send_drag(Display *display, Window window,
                               int x1, int y1, int x2, int y2,
                               unsigned int button, unsigned int steps)
{
  unsigned int state;
  unsigned int i;
  int x;
  int y;
  int ret;

  if (steps == 0)
    {
      steps = 1;
    }

  state = x11_input_button_state_mask(button);
  ret = x11_input_send_button(display, window, ButtonPress, x1, y1, button);
  if (ret != 0)
    {
      return ret;
    }

  for (i = 1; i <= steps; i++)
    {
      x = x1 + (int)(((int64_t)(x2 - x1) * (int64_t)i) /
                     (int64_t)steps);
      y = y1 + (int)(((int64_t)(y2 - y1) * (int64_t)i) /
                     (int64_t)steps);
      ret = x11_input_send_motion_state(display, window, x, y, state);
      if (ret != 0)
        {
          return ret;
        }

      usleep(20000);
    }

  return x11_input_send_button(display, window, ButtonRelease,
                               x2, y2, button);
}

static int x11_input_send_key(Display *display, Window window,
                              const char *name, bool release)
{
  XEvent event;
  KeySym sym;
  KeyCode code;

  sym = XStringToKeysym(name);
  if (sym == NoSymbol && strlen(name) == 1)
    {
      sym = (KeySym)name[0];
    }

  if (sym == NoSymbol)
    {
      fprintf(stderr, "unknown keysym: %s\n", name);
      return 1;
    }

  code = XKeysymToKeycode(display, sym);
  if (code == 0)
    {
      fprintf(stderr, "no keycode for keysym: %s\n", name);
      return 1;
    }

  x11_input_try_focus(display, window);

#ifdef HAVE_XTEST
  if (XTestFakeKeyEvent(display, code, !release, CurrentTime) != 0)
    {
      XFlush(display);
      return 0;
    }
#endif

  memset(&event, 0, sizeof(event));
  event.xkey.type = release ? KeyRelease : KeyPress;
  event.xkey.display = display;
  event.xkey.window = window;
  event.xkey.root = DefaultRootWindow(display);
  event.xkey.subwindow = None;
  event.xkey.time = CurrentTime;
  event.xkey.keycode = code;
  event.xkey.same_screen = True;

  if (XSendEvent(display, window, True,
                 release ? KeyReleaseMask : KeyPressMask, &event) == 0)
    {
      return 1;
    }

  XFlush(display);
  return 0;
}

static int x11_input_send_close(Display *display, Window window)
{
  XEvent event;
  Atom protocols;
  Atom delete_window;

  protocols = XInternAtom(display, "WM_PROTOCOLS", False);
  delete_window = XInternAtom(display, "WM_DELETE_WINDOW", False);

  memset(&event, 0, sizeof(event));
  event.xclient.type = ClientMessage;
  event.xclient.display = display;
  event.xclient.window = window;
  event.xclient.message_type = protocols;
  event.xclient.format = 32;
  event.xclient.data.l[0] = delete_window;
  event.xclient.data.l[1] = CurrentTime;

  if (XSendEvent(display, window, False, NoEventMask, &event) == 0)
    {
      return 1;
    }

  XFlush(display);
  return 0;
}

static int x11_input_send_net_close(Display *display, Window window)
{
  XEvent event;
  Atom net_close_window;

  net_close_window = XInternAtom(display, "_NET_CLOSE_WINDOW", False);

  memset(&event, 0, sizeof(event));
  event.xclient.type = ClientMessage;
  event.xclient.display = display;
  event.xclient.window = window;
  event.xclient.message_type = net_close_window;
  event.xclient.format = 32;
  event.xclient.data.l[0] = CurrentTime;
  event.xclient.data.l[1] = 2;
  event.xclient.data.l[2] = 0;

  if (XSendEvent(display, DefaultRootWindow(display), False,
                 SubstructureRedirectMask | SubstructureNotifyMask,
                 &event) == 0)
    {
      return 1;
    }

  XFlush(display);
  return 0;
}

static int x11_input_send_destroy_notify(Display *display, Window window)
{
  XEvent event;

  memset(&event, 0, sizeof(event));
  event.xdestroywindow.type = DestroyNotify;
  event.xdestroywindow.display = display;
  event.xdestroywindow.event = window;
  event.xdestroywindow.window = window;

  if (XSendEvent(display, window, False, StructureNotifyMask, &event) == 0)
    {
      return 1;
    }

  XSync(display, False);
  return 0;
}

static bool x11_input_supports_delete_window(Display *display, Window window)
{
  Atom *protocols;
  Atom delete_window;
  int count;
  int i;
  bool supported;

  protocols = NULL;
  count = 0;
  supported = false;
  delete_window = XInternAtom(display, "WM_DELETE_WINDOW", False);

  if (XGetWMProtocols(display, window, &protocols, &count) == 0)
    {
      return false;
    }

  for (i = 0; i < count; i++)
    {
      if (protocols[i] == delete_window)
        {
          supported = true;
          break;
        }
    }

  if (protocols != NULL)
    {
      XFree(protocols);
    }

  return supported;
}

static Window x11_input_find_close_window(Display *display, Window window)
{
  Window parent;
  Window root;
  Window *children;
  Window current;
  unsigned int nchildren;

  current = window;
  children = NULL;
  nchildren = 0;
  while (current != None && current != DefaultRootWindow(display))
    {
      if (x11_input_supports_delete_window(display, current))
        {
          return current;
        }

      if (XQueryTree(display, current, &root, &parent, &children,
                     &nchildren) == 0)
        {
          break;
        }

      if (children != NULL)
        {
          XFree(children);
          children = NULL;
        }

      current = parent;
    }

  return window;
}

static Window x11_input_find_root_child(Display *display, Window root,
                                        const char *needle)
{
  Window parent;
  Window root_return;
  Window *children;
  Window found;
  unsigned int nchildren;
  unsigned int i;

  children = NULL;
  nchildren = 0;
  found = None;
  if (XQueryTree(display, root, &root_return, &parent, &children,
                 &nchildren) == 0)
    {
      return None;
    }

  for (i = 0; i < nchildren; i++)
    {
      if (x11_input_name_matches(display, children[i], needle))
        {
          found = children[i];
          break;
        }
    }

  if (children != NULL)
    {
      XFree(children);
    }

  return found;
}

static int x11_input_click_frame_close(Display *display, Window frame)
{
  XWindowAttributes attrs;
  XEvent event;
  int root_x;
  int root_y;

  if (XGetWindowAttributes(display, frame, &attrs) == 0)
    {
      return 1;
    }

  if (!x11_input_translate(display, frame, attrs.width - 18, 16,
                           &root_x, &root_y))
    {
      return 1;
    }

  XRaiseWindow(display, frame);
#ifdef HAVE_XTEST
  XTestFakeMotionEvent(display, DefaultScreen(display), root_x, root_y,
                       CurrentTime);
  XTestFakeButtonEvent(display, 1, True, CurrentTime);
  XTestFakeButtonEvent(display, 1, False, CurrentTime);
  XFlush(display);
  return 0;
#else
  XWarpPointer(display, None, DefaultRootWindow(display), 0, 0, 0, 0,
               root_x, root_y);
  XSync(display, False);

  memset(&event, 0, sizeof(event));
  event.xbutton.display = display;
  event.xbutton.window = frame;
  event.xbutton.root = DefaultRootWindow(display);
  event.xbutton.subwindow = None;
  event.xbutton.time = CurrentTime;
  event.xbutton.x = attrs.width - 18;
  event.xbutton.y = 16;
  event.xbutton.x_root = root_x;
  event.xbutton.y_root = root_y;
  event.xbutton.button = 1;
  event.xbutton.same_screen = True;

  event.xbutton.type = ButtonPress;
  event.xbutton.state = 0;
  if (XSendEvent(display, frame, True, ButtonPressMask, &event) == 0)
    {
      return 1;
    }

  event.xbutton.type = ButtonRelease;
  event.xbutton.state = Button1Mask;
  if (XSendEvent(display, frame, True, ButtonReleaseMask, &event) == 0)
    {
      return 1;
    }

  XFlush(display);
  return 0;
#endif
}

static void x11_input_usage(const char *progname)
{
  fprintf(stderr,
          "usage: %s <window-name> move <x> <y>\n"
          "       %s <window-name> click <x> <y> [button]\n"
          "       %s <window-name> drag <x1> <y1> <x2> <y2> [button] [steps]\n"
          "       %s <window-name> key <keysym>\n"
          "       %s <window-name> keyup <keysym>\n"
          "       %s <window-name> close\n"
          "       %s <window-name> frameclose\n"
          "       %s <window-name> destroy\n",
          progname, progname, progname, progname, progname, progname,
          progname, progname);
}

/****************************************************************************
 * Public Functions
 ****************************************************************************/

int main(int argc, char **argv)
{
  Display *display;
  Window window;
  const char *action;
  const char *needle;
  int ret;
  int x;
  int y;
  int x2;
  int y2;
  unsigned int button;
  unsigned int steps;

  if (argc < 3)
    {
      x11_input_usage(argv[0]);
      return 2;
    }

  needle = argv[1];
  action = argv[2];

  display = XOpenDisplay(NULL);
  if (display == NULL)
    {
      fprintf(stderr, "failed to open DISPLAY\n");
      return 1;
    }

  window = x11_input_find_window(display, DefaultRootWindow(display),
                                 needle);
  if (window == None)
    {
      fprintf(stderr, "window not found: %s\n", needle);
      XCloseDisplay(display);
      return 1;
    }

  ret = 0;
  if (strcmp(action, "move") == 0)
    {
      if (argc != 5)
        {
          x11_input_usage(argv[0]);
          ret = 2;
        }
      else
        {
          x = atoi(argv[3]);
          y = atoi(argv[4]);
          ret = x11_input_send_motion(display, window, x, y);
        }
    }
  else if (strcmp(action, "click") == 0)
    {
      if (argc != 5 && argc != 6)
        {
          x11_input_usage(argv[0]);
          ret = 2;
        }
      else
        {
          x = atoi(argv[3]);
          y = atoi(argv[4]);
          button = argc == 6 ? (unsigned int)atoi(argv[5]) : 1;
          ret = x11_input_send_button(display, window, ButtonPress,
                                      x, y, button);
          if (ret == 0)
            {
              ret = x11_input_send_button(display, window, ButtonRelease,
                                          x, y, button);
            }
        }
    }
  else if (strcmp(action, "drag") == 0)
    {
      if (argc < 7 || argc > 9)
        {
          x11_input_usage(argv[0]);
          ret = 2;
        }
      else
        {
          x = atoi(argv[3]);
          y = atoi(argv[4]);
          x2 = atoi(argv[5]);
          y2 = atoi(argv[6]);
          button = argc >= 8 ? (unsigned int)atoi(argv[7]) : 1;
          steps = argc == 9 ? (unsigned int)atoi(argv[8]) : 8;
          ret = x11_input_send_drag(display, window, x, y, x2, y2,
                                    button, steps);
        }
    }
  else if (strcmp(action, "key") == 0)
    {
      if (argc != 4)
        {
          x11_input_usage(argv[0]);
          ret = 2;
        }
      else
        {
          ret = x11_input_send_key(display, window, argv[3], false);
        }
    }
  else if (strcmp(action, "keyup") == 0)
    {
      if (argc != 4)
        {
          x11_input_usage(argv[0]);
          ret = 2;
        }
      else
        {
          ret = x11_input_send_key(display, window, argv[3], true);
        }
    }
  else if (strcmp(action, "close") == 0)
    {
      window = x11_input_find_close_window(display, window);
      ret = x11_input_send_close(display, window);
      if (ret == 0)
        {
          ret = x11_input_send_net_close(display, window);
        }
    }
  else if (strcmp(action, "frameclose") == 0)
    {
      window = x11_input_find_root_child(display, DefaultRootWindow(display),
                                         needle);
      if (window == None)
        {
          fprintf(stderr, "frame window not found: %s\n", needle);
          ret = 1;
        }
      else
        {
          ret = x11_input_click_frame_close(display, window);
        }
    }
  else if (strcmp(action, "destroy") == 0)
    {
      ret = x11_input_send_destroy_notify(display, window);
    }
  else
    {
      x11_input_usage(argv[0]);
      ret = 2;
    }

  XCloseDisplay(display);
  return ret;
}
