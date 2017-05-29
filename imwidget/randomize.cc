#include "imwidget/randomize.h"

#include "imgui.h"
#include "alg/terrain.h"
#include "imwidget/map_connect.h"
#include "proto/rominfo.pb.h"
#include "util/config.h"
#include "util/stb_tilemap_editor.h"

namespace z2util {

RandomizeOverworld::RandomizeOverworld()
  : random_params_{0, true, 0.03f, Terrain::MOUNTAINS, Terrain::SAND}
{}

void RandomizeOverworld::InitParams(stbte_tilemap* editor) {
    if (random_params_.initialized)
        return;
    const auto& misc = ConfigLoader<RomInfo>::GetConfig().misc();
    int height = misc.overworld_height();
    if (!stbte_get_selection(editor,
                &random_params_.x0, &random_params_.y0,
                &random_params_.x1, &random_params_.y1)) {
        random_params_.x0 = 0;
        random_params_.y0 = 0;
        random_params_.x1 = 63;
        random_params_.y1 = height - 1;
    }
    random_params_.initialized = true;
    random_params_.keep = false;
    SaveMap(editor);
}

void RandomizeOverworld::SaveMap(stbte_tilemap* editor) {
    const auto& misc = ConfigLoader<RomInfo>::GetConfig().misc();
    int height = misc.overworld_height();
    for(int y=0; y<height; y++) {
        for(int x=0; x<64; x++) {
            random_params_.backup[y][x] = stbte_get_tile(editor, x, y)[0];
        }
    }
}

void RandomizeOverworld::RestoreMap(stbte_tilemap* editor) {
    const auto& misc = ConfigLoader<RomInfo>::GetConfig().misc();
    int height = misc.overworld_height();
    for(int y=0; y<height; y++) {
        for(int x=0; x<64; x++) {
            stbte_set_tile(editor, x, y, 0, random_params_.backup[y][x]);
        }
    }
}

bool RandomizeOverworld::Draw(stbte_tilemap* editor, OverworldConnectorList* connections) {
    bool changed = false;
    const char * algorithms = "Perlin\0Cellular\0\0";
    const char * terrains = "Town\0Cave\0Palace\0Bridge\0Sand\0Grass\0Forest\0Swamp\0Graveyard\0Road\0Lava\0Mountain\0Water\0Walk-Water\0Boulder\0Spider\0\0";

    if (ImGui::BeginPopup("Randomize")) {
        InitParams(editor);
        ImGui::Text("Randomize Overworld Map");

        ImGui::PushItemWidth(100);
        ImGui::InputInt("x0", &random_params_.x0);
        ImGui::SameLine();
        ImGui::InputInt("x1", &random_params_.x1);

        ImGui::InputInt("y0", &random_params_.y0);
        ImGui::SameLine();
        ImGui::InputInt("y1", &random_params_.y1);

        ImGui::InputInt("cx", &random_params_.centerx);
        ImGui::SameLine();
        ImGui::InputInt("cy", &random_params_.centery);
        ImGui::PopItemWidth();

        ImGui::Combo("Algorithm", &random_params_.algorithm, algorithms);
        ImGui::InputInt("Seed", &random_params_.seed);

        int x0 = random_params_.x0;
        int y0 = random_params_.y0;
        int x1 = random_params_.x1;
        int y1 = random_params_.y1;
        int cx = random_params_.centerx;
        int cy = random_params_.centery;

        auto alg = Terrain::Type(random_params_.algorithm);
        auto terrain = Terrain::New(alg);
        if (alg == Terrain::PERLIN) {
            ImGui::SliderFloat("NoiseZoom", &random_params_.p, 0.0f, 1.0f);
            static_cast<PerlinTerrain*>(terrain.get())->set_noise_zoom(random_params_.p);
            cx += x0 + (x1-x0) / 2;
            cy += y0 + (y1-y0) / 2;
        } else {
            ImGui::SliderFloat("Probability", &random_params_.p, 0.0f, 1.0f);
            ImGui::PushItemWidth(100);
            ImGui::Combo("BG", &random_params_.bg, terrains);
            ImGui::SameLine();
            ImGui::Combo("FG", &random_params_.fg, terrains);
            ImGui::PopItemWidth();
            static_cast<CellularTerrain*>(terrain.get())->set_probability(random_params_.p);
            static_cast<CellularTerrain*>(terrain.get())->set_bg(random_params_.bg);
            static_cast<CellularTerrain*>(terrain.get())->set_fg(random_params_.fg);
        }

        ImGui::Checkbox("Keep Transfer Tiles", &random_params_.keep_transfer_tiles);
        terrain->Generate(unsigned(random_params_.seed));
        for(int y=y0; y<=y1; y++) {
            for(int x=x0; x<=x1; x++) {
                stbte_set_tile(editor, x, y, 0, terrain->GetTile(x-cx, y-cy));
                if (random_params_.keep_transfer_tiles
                    && connections->GetAtXY(x, y)) {
                    stbte_set_tile(editor, x, y, 0, random_params_.backup[y][x]);
                }
            }
        }
        if (ImGui::Button("Apply")) {
            ImGui::CloseCurrentPopup();
            random_params_.keep = true;
            changed = true;
        }
        ImGui::SameLine();
        if (ImGui::Button("Cancel")) {
            ImGui::CloseCurrentPopup();
        }
        ImGui::EndPopup();
    } else {
        if (random_params_.initialized) {
            if (!random_params_.keep) {
                RestoreMap(editor);
            }
            random_params_.initialized = false;
        }
    }
    return changed;
}

}  // namespace z2util
