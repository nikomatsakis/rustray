import std::io;
import std::io::writer_util;
import std::io::writer;
import raytracer::*;

const WIDTH : uint = 512u;
const HEIGHT : uint = 512u;
const FOV : float = 3.14159f / 3f ;

fn write_ppm( fname: str, width: uint, height: uint, pixels: [color] ){

	let writer = result::get( io::file_writer( fname, [io::create, io::truncate] ) );

	writer.write_str(#fmt("P6\n%u %u\n255\n", width, height));
	for pixel in pixels{
		writer.write([pixel.r, pixel.g, pixel.b]);
	};
}

fn main( _args: [str] )
{
	
	let model = model::read_model( "models/cow-nonormals.obj" );
	let {depth,count} = model::count_kd_tree_nodes( model.kd_tree );

	io::println(#fmt("Done.\nLoaded model.\n\tVerts: %u, Tris: %u\n\tKD-tree depth: %u, #nodes: %u", 
				vec::len(model.mesh.vertices), 
				vec::len(model.mesh.indices)/3u,
				depth, count));

	io::print("Tracing rays... ");
	let pixels = raytracer::generate_raytraced_image(model, FOV, WIDTH, HEIGHT);
	io::println("Done!");
	io::print("Writing file... ");
	write_ppm( "oput.ppm", WIDTH, HEIGHT, pixels );	
	io::println("Done!");
}
