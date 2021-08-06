#include <iostream>
#include <thread>
#include <chrono>
#include <nanogui/nanogui.h>
using namespace std::literals::chrono_literals;

class levelBuilderApp : public nanogui::Screen {
	public:
		levelBuilderApp() : nanogui::Screen(nanogui::Vector2i(WINDOW_WIDTH, WINDOW_HEIGHT), "Levelbuilder"){
			using namespace nanogui;
			this->setBackground(Color(0, 0, 0, 0));
			Window* optionsBar = new Window(this, "");
			optionsBar->setPosition(Vector2i(0, 0));
			optionsBar->setSize(Vector2i(WINDOW_WIDTH, WINDOW_HEIGHT/30));
			optionsBar->setPosition({0, 0});
			otionsBar->mdrag = false;
			//auto b = new Button(optionsBar->buttonPanel(), "Hello");
			
			//b = new Button(optionsBar->buttonPanel(), "Hello1");
			
			//nanogui::ImagePanel view = new nanogui::ImagePanel(app);
		}
		virtual ~levelBuilderApp(){
			std::cout << "Goodbye" << std::endl;
		}
		
		static constexpr int WINDOW_WIDTH = 1920;
		static constexpr int WINDOW_HEIGHT = 1080;
	private:
};


int main(){
		nanogui::init();
		nanogui::ref<levelBuilderApp> app = new levelBuilderApp();

		//nanogui::Window* win = new nanogui::Window(app, "test");
		/*win->setPosition(nanogui::Vector2i(15, 15));
        win->setLayout(new nanogui::GroupLayout());
		win->center();*/
		
		app->drawAll();
		app->performLayout();
		app->setVisible(true);
		nanogui::mainloop();
		app->setVisible(false);
		std::this_thread::sleep_for(1s);
		nanogui::shutdown();
		return 0;
}