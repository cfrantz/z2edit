#include "music.h"

#include <iostream>

void dump_notes(std::vector<z2music::Note> notes) {
  for (z2music::Note n : notes) {
    std::cout << n.pitch_string();
    size_t left = n.length() / 4 - 4;
    std::cout << std::string(left, '.');
  }
  std::cout << std::endl;
}

void dump_song(const z2music::Song& song) {
  std::cout << "Song length: " << song.sequence_length() << " phrases" << std::endl;

  for (size_t i = 0; i < song.sequence_length(); ++i) {
    const z2music::Pattern* p = song.at(i);
    dump_notes(p->notes(z2music::Pattern::Channel::Pulse1));
    dump_notes(p->notes(z2music::Pattern::Channel::Pulse2));
    dump_notes(p->notes(z2music::Pattern::Channel::Triangle));
    dump_notes(p->notes(z2music::Pattern::Channel::Noise));
    std::cout << std::endl;
  }
}

int main(int argc, char** argv) {
  if (argc != 2) {
    std::cout << "Usage: " << argv[0] << " z2_rom" << std::endl;
    return 1;
  }

  const std::string file = std::string(argv[1]);

  z2music::Rom rom(file);
  z2music::Song* battle = rom.song(z2music::Rom::SongTitle::BattleTheme);
  dump_song(*battle);

  battle->clear();

  z2music::Pattern a;
  z2music::Pattern b;

  const z2music::Note CE(z2music::Note::Duration::Eighth, z2music::Note::Pitch::C4);
  const z2music::Note CH(z2music::Note::Duration::Half, z2music::Note::Pitch::C4);
  const z2music::Note DE(z2music::Note::Duration::Eighth, z2music::Note::Pitch::D4);
  const z2music::Note DQ(z2music::Note::Duration::Quarter, z2music::Note::Pitch::D4);
  const z2music::Note EQ(z2music::Note::Duration::Quarter, z2music::Note::Pitch::E4);

  a.add_notes(z2music::Pattern::Channel::Pulse1, { EQ, DQ, CH });
  b.add_notes(z2music::Pattern::Channel::Pulse1, { CE, CE, CE, CE, DE, DE, DE, DE });

  battle->add_pattern(a);
  battle->add_pattern(b);
  battle->set_sequence({0, 0, 1, 0});

  dump_song(*battle);

  rom.save("/tmp/output.nes");

  return 0;
}
