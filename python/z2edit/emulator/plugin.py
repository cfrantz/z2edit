class Plugin(object):
    def __init__(self, emulator):
        self.emulator = emulator

    def run_pre_frame(self):
        pass

    def run_per_frame(self):
        pass

    def draw_image(self, origin):
        pass

    def draw(self):
        pass

    def menu_bar(self):
        pass

    def menu(self, name):
        pass
