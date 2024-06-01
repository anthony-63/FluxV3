#pragma once

#include <godot_cpp/classes/node.hpp>
#include <godot_cpp/classes/label.hpp>
#include <godot_cpp/classes/progress_bar.hpp>

namespace godot {
    class Startup : public Node {
        GDCLASS(Startup, Node)

    protected:
        static void _bind_methods();

    public:
        Startup();
        ~Startup();
        
        virtual void _ready() override;

        Label *stage_label;
        Label *substage_label;
        ProgressBar *progress;

        std::thread loading_thread;
    };
}